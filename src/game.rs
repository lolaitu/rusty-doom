use std::io::{Write, Result};
use std::time::{Duration, Instant};
use crossterm::{
  event::{self, KeyEvent},
  terminal::{self, size},
};
use crate::level::Level;
use crate::player::Player;
use crate::world::World;
use crate::entity::{Entity, EntityType, SpriteType};
use crate::weapon::Weapon;
use crate::graphics;
use crate::input::{InputManager, Action};

pub struct Game {
  pub time_of_launch: Instant,
  pub time_of_last_loop: Instant,
  pub player: Player,
  pub player_entity_id: u32,
  pub world: World,
  pub level: Level,
  pub term_size: (u16, u16),
  pub target_fps: u32,
  pub fps_toggle_cooldown: Instant,
}

impl Game {
  pub fn new(level: Level) -> Result<Self> {
    let (cols, rows) = size().unwrap();
    let mut world = World::new();
    
    // Create player entity
    let player_entity = Entity::new_player(0, 3.5, 3.5);
    let player_id = world.spawn_entity(player_entity);
    
    // Create player struct
    let player = Player::new()?;

    // Spawn some enemies
    let imp = Entity::new_enemy(0, 10.5, 10.5, SpriteType::EnemyImp);
    world.spawn_entity(imp);
    
    let demon = Entity::new_enemy(0, 7.5, 7.5, SpriteType::EnemyDemon);
    world.spawn_entity(demon);

    Ok(Self {
      time_of_launch: Instant::now(),
      time_of_last_loop: Instant::now(),
      player,
      player_entity_id: player_id,
      world,
      level,
      term_size: (cols, rows),
      target_fps: 30,
      fps_toggle_cooldown: Instant::now(),
    })
  }

  pub fn update(&mut self, render_buffer: &mut graphics::RenderBuffer, input_manager: &InputManager) -> Result<bool> {
    let mut write = std::io::stdout();

    self.term_size = terminal::size()?;

    // Calculate delta time
    let now = Instant::now();
    let delta_time = now.duration_since(self.time_of_last_loop).as_secs_f64();
    self.time_of_last_loop = now;

    // Handle inputs
    let mut move_speed = self.player.max_speed;
    let mut rot_speed = self.player.max_rotation_speed;

    // Apply movement penalties when firing
    if self.player.is_firing() {
        let penalty = self.player.get_current_weapon().movement_penalty;
        move_speed *= 1.0 - penalty;
        rot_speed *= 1.0 - penalty;
    }

    // Toggle FPS
    if input_manager.is_active(Action::ToggleFPS) {
        if now >= self.fps_toggle_cooldown {
            if self.target_fps == 60 {
                self.target_fps = 30;
            } else {
                self.target_fps = 60;
            }
            self.fps_toggle_cooldown = now + Duration::from_millis(500);
        }
    }

    if input_manager.is_active(Action::MoveForward) {
        self.world.move_entity_forward(self.player_entity_id, move_speed * delta_time, &self.level);
    }
    if input_manager.is_active(Action::MoveBackward) {
        self.world.move_entity_forward(self.player_entity_id, -move_speed * delta_time, &self.level);
    }
    if input_manager.is_active(Action::StrafeRight) {
        self.world.strafe_entity(self.player_entity_id, move_speed * delta_time, &self.level);
    }
    if input_manager.is_active(Action::StrafeLeft) {
        self.world.strafe_entity(self.player_entity_id, -move_speed * delta_time, &self.level);
    }
    if input_manager.is_active(Action::RotateLeft) {
        self.world.rotate_entity(self.player_entity_id, rot_speed * delta_time);
    }
    if input_manager.is_active(Action::RotateRight) {
        self.world.rotate_entity(self.player_entity_id, -rot_speed * delta_time);
    }
    if input_manager.is_active(Action::Shoot) {
        if self.player.fire() {
          // Spawn projectile(s)
          let mut spawn_data = None;
          
          if let Some(player) = self.world.get_entity(self.player_entity_id) {
            let weapon = self.player.get_current_weapon();
            let count = weapon.projectile_count;
            let spread = weapon.spread;
            let damage = weapon.damage;
            let range = weapon.range;
            
            // Determine sprite type based on weapon
            let sprite_type = match weapon.weapon_type {
                crate::weapon::WeaponType::Pistol => crate::entity::SpriteType::ProjectilePistol,
                crate::weapon::WeaponType::Shotgun => crate::entity::SpriteType::ProjectileShotgun,
                crate::weapon::WeaponType::Gatling => crate::entity::SpriteType::ProjectileGatling,
            };

            let radians = player.transform.angle.to_radians();
            let gun_offset = 0.5; // Distance from player center to gun barrel
            let spawn_x = player.transform.x + radians.cos() * gun_offset;
            let spawn_y = player.transform.y + radians.sin() * gun_offset;
            let angle = player.transform.angle;
            
            spawn_data = Some((spawn_x, spawn_y, angle, count, spread, damage, range, sprite_type));
          }
          
          if let Some((spawn_x, spawn_y, angle, count, spread, damage, range, sprite_type)) = spawn_data {
            for i in 0..count {
                let angle_offset = if count > 1 {
                    (i as f64 - (count as f64 - 1.0) / 2.0) * spread
                } else {
                    0.0
                };
                
                self.world.spawn_projectile(spawn_x, spawn_y, angle + angle_offset, damage, range, sprite_type);
            }
          }
        }
    }
    if input_manager.is_active(Action::Reload) {
        self.player.reload();
    }
    if input_manager.is_active(Action::RespawnEnemies) {
        self.world.respawn_enemies();
    }
    if input_manager.is_active(Action::SwitchWeapon1) {
        self.player.switch_weapon(0);
    }
    if input_manager.is_active(Action::SwitchWeapon2) {
        self.player.switch_weapon(1);
    }
    if input_manager.is_active(Action::SwitchWeapon3) {
        self.player.switch_weapon(2);
    }

    if input_manager.is_active(Action::Quit) {
        return Ok(true);
    }

    // Sync player struct with entity
    if let Some(entity) = self.world.get_entity(self.player_entity_id) {
        self.player.health = entity.health as u32;
    }

    // Update world physics
    self.world.update(delta_time, &self.level);
    
    // Update player animation
    self.player.animation_update();
    
    // Frame limiting
    let target_duration = Duration::from_secs_f64(1.0 / self.target_fps as f64);
    let elapsed = self.time_of_last_loop.elapsed();
    if elapsed < target_duration {
        std::thread::sleep(target_duration - elapsed);
    }

    // Render 3D raycasting view
    let mut stdout = std::io::stdout();
    graphics::draw(self, render_buffer)?;
    render_buffer.flush(&mut stdout)?;

    write.flush()?;

    Ok(false) // Continue running
  }

}
