use std::io::{Write, Result};
use std::time::{Duration, Instant};
use crossterm::{
  event::{self, KeyEvent},
  terminal,
};
use crate::level::Level;
use crate::player::{Joueur, PlayerInput};
use crate::world::World;
use crate::entity::Entity;

pub struct Game {
  pub time_of_launch: Instant,
  pub time_of_last_loop: Instant,
  pub time_delta: Duration,

  pub level: Level,
  pub world: World,
  pub joueur: Joueur,
  pub player_entity_id: u32,

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

    Ok(Self {
      time_of_launch: now,
      time_of_last_loop: now,
      time_delta: Duration::ZERO,
      level,
      world,
      joueur,
      player_entity_id,
      term_size: terminal::size()?,
    })
  }

  pub fn handle_input(&mut self, key_event: KeyEvent) -> Result<()> {
    let input = self.joueur.process_input(key_event);
    
    match input {
      PlayerInput::MoveForward => {
        self.world.move_entity_forward(self.player_entity_id, self.joueur.max_speed, &self.level);
      }
      PlayerInput::MoveBackward => {
        self.world.move_entity_forward(self.player_entity_id, -self.joueur.max_speed, &self.level);
      }
      PlayerInput::StrafeRight => {
        self.world.strafe_entity(self.player_entity_id, self.joueur.max_speed, &self.level);
      }
      PlayerInput::StrafeLeft => {
        self.world.strafe_entity(self.player_entity_id, -self.joueur.max_speed, &self.level);
      }
      PlayerInput::RotateLeft => {
        self.world.rotate_entity(self.player_entity_id, self.joueur.max_rotation_speed);
      }
      PlayerInput::RotateRight => {
        self.world.rotate_entity(self.player_entity_id, -self.joueur.max_rotation_speed);
      }
      PlayerInput::None => {}
    }
    
    Ok(())
  }

  pub fn update(&mut self) -> Result<bool> {
    let mut write = std::io::stdout();

    self.term_size = terminal::size()?;

    self.time_delta = self.time_of_last_loop.elapsed();
    self.time_of_last_loop = Instant::now();

    // Update world physics
    self.world.update(self.time_delta.as_secs_f64());

    // Cap at ~35 FPS like original Doom
    std::thread::sleep(Duration::from_millis(28));

    // Get player entity for rendering
    if let Some(player_entity) = self.world.get_entity(self.player_entity_id) {
      self.level.print_with_player_entity(player_entity)?;
    }

    write.flush()?;

    Ok(false) // Continue running
  }

}
