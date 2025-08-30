use std::io::{Write, Result};
use std::time::{Duration, Instant};
use crossterm::{
  event::{self, KeyEvent},
  terminal,
};
use crate::level::Level;
use crate::player::Joueur;

pub struct Game {
  pub time_of_launch: Instant,
  pub time_of_last_loop: Instant,
  pub time_delta: Duration,

  pub level: Level,
  pub joueur : Joueur,

  pub term_size: (u16, u16),
}

impl Game {
  pub fn new(level: Level) -> Result<Self> {
    let now = Instant::now();
    Ok( Self {
      time_of_launch: now,
      time_of_last_loop: now,
      time_delta: Duration::ZERO,
      level: level,
      joueur : Joueur::new()?,
      term_size: terminal::size()?,
    })
  }

  pub fn handle_input(&mut self, key_event: KeyEvent) -> Result<()> {
    self.joueur.handle_input(key_event, &self.level)?;
    Ok(())
  }

  pub fn update(&mut self) -> Result<bool> {
    let mut write = std::io::stdout();

    self.term_size = terminal::size()?;

    self.time_delta = self.time_of_last_loop.elapsed();
    self.time_of_last_loop = Instant::now();

    // Cap at ~35 FPS like original Doom
    std::thread::sleep(Duration::from_millis(28));

    self.level.print_with_player(&self.joueur)?;

    write.flush()?;

    Ok(false) // Continue running
  }

}
