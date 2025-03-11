use std::io::{stdout,Write, Result};
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal,
    execute,
    terminal::{Clear,ClearType}
};
use crate::graphics::draw;
use crate::level::{self, Level};
use crate::joueur::Joueur;
use crate::joueur;



pub struct Game {
	pub time_of_launch: Instant,
	pub time_of_last_loop: Instant,
	pub time_delta: Duration,

	pub term_size: (u16, u16),
  pub level : Level,
  pub stdout: std::io::Stdout,
}

impl Game {
    pub fn new() -> Result<Self> {
        let now = Instant::now();
        Ok( Self {
            time_of_launch: now,
            time_of_last_loop: now,
            time_delta: Duration::ZERO,
            term_size: terminal::size()?,
            level : level::Level::debug_1()?,
            stdout : stdout()
        })
    }

    pub fn launch(&mut self) -> Result<()> {
        let mut joueur = joueur::Joueur::new()?;
        loop {
            self.main_loop(& mut joueur)?;
            
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

fn main_loop(&mut self, Joueur : &mut Joueur) -> Result<()> {

        let mut write = std::io::stdout();

        //self.term_size = terminal::size()?;

        self.time_delta = self.time_of_last_loop.elapsed();
        self.time_of_last_loop += self.time_delta;

        execute!(self.stdout, Clear(ClearType::All))?;
        draw(self,Joueur, &mut self.stdout);

        write.flush()?;

        Ok(())
    }
}

