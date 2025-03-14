use std::io::{stdout,Write, Result};
use std::time::{Duration, Instant};
use crossterm::{
  event::{self, Event, KeyCode, KeyModifiers},
  terminal,
};
use crate::graphics::draw;
use crate::level::Level;
use crate::joueur::Joueur;
use crate::joueur;



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
      joueur : joueur::Joueur::new()?,
      term_size: terminal::size()?,
    })
  }

  pub fn launch(&mut self) -> Result<()> {
    let mut stdout = stdout();
    loop {
      self.main_loop(&mut stdout)?;
      
      // Ctrl-C to close the loop
      if event::poll(Duration::from_millis(50))?{
        if let Event::Key(key_event) = event::read()? {
          if key_event.code == KeyCode::Char('c') &&
            key_event.modifiers.contains(KeyModifiers::CONTROL)
          { break; }
        }
      }

    }
    Ok(())
  }

  fn main_loop(&mut self, stdout : &mut std::io::Stdout) -> Result<()> {

    let mut write = std::io::stdout();

    self.term_size = terminal::size()?;

    self.time_delta = self.time_of_last_loop.elapsed();
    self.time_of_last_loop = Instant::now();

    //execute!(write, Clear(ClearType::All))?;
    //draw(self, stdout)?;
    self.level.print_with_player(&self.joueur)?;

    self.joueur.update()?;

    write.flush()?;

    Ok(())
  }
}

