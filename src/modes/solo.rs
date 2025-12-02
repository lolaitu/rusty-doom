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
use crate::graphics::RenderBuffer;
use crate::graphics::draw;
use crate::input::{InputManager, Action};
use crate::common::physics::Physics;

pub struct SoloGame {
    pub time_of_launch: Instant,
    pub time_of_last_loop: Instant,
    pub player: Player,
    pub player_id: u32,
    pub world: World,
    pub level: Level,
    pub term_size: (u16, u16),
    pub target_fps: u32,
    pub fps_toggle_cooldown: Instant,
    pub render_buffer: RenderBuffer,
}

impl SoloGame {
    pub fn new(level: Level) -> Result<Self> {
        // Initialize render buffer
        let mut term_size = terminal::size()?;
        let mut render_buffer = RenderBuffer::new(term_size.0, term_size.1);

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
        player_id: player_id,
        world,
        level,
        term_size,
        target_fps: 30,
        fps_toggle_cooldown: Instant::now(),
        render_buffer,
        })
    }
}

use crate::modes::gamemode::GameMode;

impl GameMode for SoloGame {
    fn update(&mut self, input_manager: &InputManager) -> Result<bool> {
        // Handle Input
        if input_manager.is_active(Action::Quit) {
            return Ok(true);
        }

        let mut write = std::io::stdout();
        
        // Store terminal size for rendering
        self.term_size = terminal::size()?;

        // Calculate delta time
        let now = Instant::now();
        let delta_time = now.duration_since(self.time_of_last_loop).as_secs_f64();
        self.time_of_last_loop = now;

        // Player Movement
        let move_speed = if input_manager.is_active(Action::Sprint) { 
            crate::common::entity::PLAYER_SPEED * 2.0 
        } else { 
            crate::common::entity::PLAYER_SPEED 
        };
        
        let rot_speed = crate::common::entity::PLAYER_ROTATION_SPEED;

        // Player Movement
        let mut move_speed = if input_manager.is_active(Action::Sprint) { 
            crate::common::entity::PLAYER_SPEED * 2.0 
        } else { 
            crate::common::entity::PLAYER_SPEED 
        };
        
        let mut rot_speed = crate::common::entity::PLAYER_ROTATION_SPEED;

        // Apply weapon weight penalty
        let penalty = self.player.get_current_weapon().movement_penalty;
        move_speed *= 1.0 - penalty;
        rot_speed *= 1.0 - (penalty * 0.5); // Less penalty on rotation

        if input_manager.is_active(Action::MoveForward) {
            Physics::move_entity_forward(&mut self.world, self.player_id, move_speed * delta_time, &self.level);
        }
        if input_manager.is_active(Action::MoveBackward) {
            Physics::move_entity_forward(&mut self.world, self.player_id, -move_speed * delta_time, &self.level);
        }
        if input_manager.is_active(Action::RotateLeft) {
            Physics::rotate_entity(&mut self.world, self.player_id, -rot_speed * delta_time);
        }
        if input_manager.is_active(Action::RotateRight) {
            Physics::rotate_entity(&mut self.world, self.player_id, rot_speed * delta_time);
        }
        if input_manager.is_active(Action::StrafeLeft) {
            Physics::strafe_entity(&mut self.world, self.player_id, -move_speed * delta_time, &self.level);
        }
        if input_manager.is_active(Action::StrafeRight) {
            Physics::strafe_entity(&mut self.world, self.player_id, move_speed * delta_time, &self.level);
        }
        
        // Sync player struct with entity (for rendering)
        if let Some(entity) = self.world.get_entity(self.player_id) {
            self.player.transform = entity.transform;
            self.player.health = entity.health as u32;
        }

        // Weapon handling
        if input_manager.is_active(Action::Shoot) {
            if self.player.fire() {
                // Spawn projectile(s)
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

                let radians = self.player.transform.angle.to_radians();
                let gun_offset = 0.5; // Distance from player center to gun barrel
                let spawn_x = self.player.transform.x + radians.cos() * gun_offset;
                let spawn_y = self.player.transform.y + radians.sin() * gun_offset;
                let angle = self.player.transform.angle;
            
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
        if input_manager.is_active(Action::Reload) {
            self.player.reload();
        }
        
        // Weapon switching
        if input_manager.is_active(Action::SwitchWeapon1) { self.player.switch_weapon(0); }
        if input_manager.is_active(Action::SwitchWeapon2) { self.player.switch_weapon(1); }
        if input_manager.is_active(Action::SwitchWeapon3) { self.player.switch_weapon(2); }

        // Update world physics
        let kills = Physics::update(&mut self.world, delta_time, &self.level);
        self.player.kills += kills;
        
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
        draw(&self.world, &self.player, &self.level, self.term_size, &mut self.render_buffer)?;
        self.render_buffer.flush(&mut stdout)?;

        write.flush()?;

        Ok(false)
    }
}