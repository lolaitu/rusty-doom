use std::io::{Write, Result};
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal,
};


use crate::graphics::draw;



pub struct Game {
	pub time_of_launch: Instant,
	pub time_of_last_loop: Instant,
	pub time_delta: Duration,

	pub term_size: (u16, u16),
}

impl Game {
    pub fn new() -> Result<Self> {
        let now = Instant::now();
        Ok( Self {
            time_of_launch: now,
            time_of_last_loop: now,
            time_delta: Duration::ZERO,
            term_size: terminal::size()?,
        })
    }

    pub fn launch(&mut self) -> Result<()> {
        loop {
            self.main_loop()?;

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

    fn main_loop(&mut self) -> Result<()> {
        let mut write = std::io::stdout();

        //self.term_size = terminal::size()?;

        self.time_delta = self.time_of_last_loop.elapsed();
        self.time_of_last_loop += self.time_delta;


        draw(self)?;

        write.flush()?;

        Ok(())
    }
}

