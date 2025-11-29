use std::io::{Write, Result};
use std::time::{Duration, Instant};
use crossterm::{
  event::{self, KeyEvent},
  terminal,
};
use crate::level::Level;
use crate::player::Joueur;
use crate::world::World;
use crate::entity::Entity;
use crate::weapon::Weapon;
use crate::graphics;
use crate::input::{InputManager, Action};

pub struct Game {
  pub time_of_launch: Instant,
  pub time_of_last_loop: Instant,
  pub time_delta: Duration,

  pub level: Level,
  pub world: World,
  pub joueur: Joueur,
  pub player_entity_id: u32,
  pub weapon: Weapon,

  pub term_size: (u16, u16),
}

impl Game {
  pub fn new(level: Level) -> Result<Self> {
    let now = Instant::now();
    let mut world = World::new();
    let joueur = Joueur::new()?;
    
    // Spawn player entity in world
    let player_entity = Entity::new_player(0, 4.0, 11.0);
    let player_entity_id = world.spawn_entity(player_entity);
    
    // Spawn some test enemies
    world.spawn_enemy(10.0, 15.0, crate::entity::SpriteType::EnemyImp);
    world.spawn_enemy(8.0, 8.0, crate::entity::SpriteType::EnemyDemon);
    world.spawn_enemy(18.0, 12.0, crate::entity::SpriteType::EnemyImp);

    Ok(Self {
      time_of_launch: now,
      time_of_last_loop: now,
      time_delta: Duration::ZERO,
      level,
      world,
      joueur,
      player_entity_id,
      weapon: Weapon::new_pistol(),
      term_size: terminal::size()?,
    })
  }

  pub fn update(&mut self, render_buffer: &mut graphics::RenderBuffer, input_manager: &InputManager) -> Result<bool> {
    let mut write = std::io::stdout();

    self.term_size = terminal::size()?;

    self.time_delta = self.time_of_last_loop.elapsed();
    self.time_of_last_loop = Instant::now();

    // Handle inputs
    if input_manager.is_active(Action::MoveForward) {
        self.world.move_entity_forward(self.player_entity_id, self.joueur.max_speed, &self.level);
    }
    if input_manager.is_active(Action::MoveBackward) {
        self.world.move_entity_forward(self.player_entity_id, -self.joueur.max_speed, &self.level);
    }
    if input_manager.is_active(Action::StrafeRight) {
        self.world.strafe_entity(self.player_entity_id, self.joueur.max_speed, &self.level);
    }
    if input_manager.is_active(Action::StrafeLeft) {
        self.world.strafe_entity(self.player_entity_id, -self.joueur.max_speed, &self.level);
    }
    if input_manager.is_active(Action::RotateLeft) {
        self.world.rotate_entity(self.player_entity_id, self.joueur.max_rotation_speed);
    }
    if input_manager.is_active(Action::RotateRight) {
        self.world.rotate_entity(self.player_entity_id, -self.joueur.max_rotation_speed);
    }
    if input_manager.is_active(Action::Shoot) {
        if self.weapon.fire() {
          // Spawn projectile from gun barrel position (slightly forward from player)
          if let Some(player) = self.world.get_entity(self.player_entity_id) {
            let radians = player.transform.angle.to_radians();
            let gun_offset = 0.5; // Distance from player center to gun barrel
            let spawn_x = player.transform.x + radians.cos() * gun_offset;
            let spawn_y = player.transform.y + radians.sin() * gun_offset;
            
            self.world.spawn_projectile(spawn_x, spawn_y, player.transform.angle);
          }
        }
    }
    if input_manager.is_active(Action::Reload) {
        self.weapon.reload();
    }
    if input_manager.is_active(Action::RespawnEnemies) {
        self.world.respawn_enemies();
    }

    // Update world physics
    self.world.update(self.time_delta.as_secs_f64(), &self.level);
    
    // Update weapon animation
    self.weapon.update();

    // Cap at ~35 FPS like original Doom
    std::thread::sleep(Duration::from_millis(28));

    // Render 3D raycasting view
    let mut stdout = std::io::stdout();
    graphics::draw(self, render_buffer)?;
    render_buffer.flush(&mut stdout)?;

    write.flush()?;

    Ok(false) // Continue running
  }

}
