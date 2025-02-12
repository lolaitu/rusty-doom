use std::io::{Stdout, Write, Result};
use std::time::{Duration, Instant};
use crossterm::{
    queue,
    cursor::{MoveTo},
    event::{self, Event, KeyCode, KeyModifiers},
    style::{Print, SetBackgroundColor, Color},
    terminal,
};


pub struct Game {
	time_of_launch: Instant,
	time_of_last_loop: Instant,
	time_delta: Duration,

	write: Stdout,
}

impl Game {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            time_of_launch: now,
            time_of_last_loop: now,
            time_delta: Duration::ZERO,
            write: std::io::stdout(),
        }
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

        let term_size = terminal::size()?;

        self.time_delta = self.time_of_last_loop.elapsed();
        self.time_of_last_loop += self.time_delta;

        for j in 0..term_size.1 {
            for i in 0..term_size.0 {
                let uv = {(
                    i as f32 / term_size.0 as f32,
                    j as f32 / term_size.1 as f32
                )};

                queue!(self.write,
                    MoveTo(i, j),
                    SetBackgroundColor(Color::Rgb{
                        r: 0_u8,
                        g: 0_u8,
                        b: ( ((self.time_of_launch.elapsed().as_millis() as f32 / 1000_f32).sin() + 1_f32) * 128_f32 ) as u8
                        //b: ((self.time_of_launch.elapsed().as_millis() as f32 / 10_f32) as u32 % 255) as u8
                    }),
                    Print(' ')
                )?;
            }
        }

        self.write.flush()?;

        Ok(())
    }
}

