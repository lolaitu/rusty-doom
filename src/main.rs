/* Main file that manages : terminal initialization, cleanup, 
rendering, input handling and main loop. */

// terminal manipulation library
use crossterm::{
  execute,
  cursor::{Hide, Show},
  terminal::{self,
    EnterAlternateScreen,
    LeaveAlternateScreen
  },
  event::{self, Event, KeyCode, KeyModifiers},
};

// result type for I/O operations
use std::io::Result;
// time type to represent span of time
use std::time::Duration;

mod game;
use game::Game;

mod graphics;

mod level;
mod player;
mod entity;
mod world;
mod weapon;


fn main() -> Result<()>{
  terminal_init()?;

  let level = level::Level::debug_1()?;
  let mut game = Game::new(level)?;
  
  // main program loop
  loop {
    // check for Ctrl+C in main
    if event::poll(Duration::from_millis(1))? {
      if let Event::Key(key_event) = event::read()? {
        if key_event.code == KeyCode::Char('c') &&
          key_event.modifiers.contains(KeyModifiers::CONTROL)
        {
          break;
        }
        // pass non-system keys to game
        game.handle_input(key_event)?;
      }
    }
    // check if the game is over
    if game.update()? {
      break;
    }
  }
  // clean the terminal before ending the program
  terminal_cleanup()?;

  Ok(())
}

fn terminal_init() -> Result<()> {
  terminal::enable_raw_mode()?;

  execute!(std::io::stdout(),
    EnterAlternateScreen,
    Hide
  )?;

  Ok(())
}

fn terminal_cleanup() -> Result<()> {
  execute!(std::io::stdout(),
    LeaveAlternateScreen,
    Show
  )?;

  terminal::disable_raw_mode()?;

  Ok(())
}