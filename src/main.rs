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
use graphics::RenderBuffer;
use graphics::draw;
use graphics::sprites;

mod common;
use common::level;
use common::entity;
use common::world;
use common::input;

use input::InputManager;
use input::Action;

mod weapon;
use weapon::Weapon;

mod player;
use player::Player;

// Main program loop
fn main() -> Result<()>{
  // Initialize terminal
  terminal_init()?;

  // Load level
  let level = level::Level::debug_1()?;
  // Create game instance
  let mut game = Game::new(level)?;

  // Get the terminal size
  let (w, h) = terminal::size()?;
  // Create a render buffer
  let mut render_buffer = RenderBuffer::new(w, h);
  
  let mut input_manager = InputManager::new();

  // Main game loop
  loop {
    // Poll inputs using device_query
    input_manager.update();
    
    while event::poll(Duration::from_millis(0))? {
        let _ = event::read()?; // Drain events
    }
    
    // Check for Ctrl+C
    if input_manager.is_active(Action::Quit) {
        break;
    }
    
    // update the game and check if it is over
    if game.update(&mut render_buffer, &input_manager)? {
      break;
    }
  }

  // clean the terminal before ending the program
  terminal_cleanup()?;
  Ok(())
}

fn terminal_init() -> Result<()> {
  // Enable raw mode
  terminal::enable_raw_mode()?;

  // Enter alternate screen
  execute!(std::io::stdout(),
    EnterAlternateScreen,
    Hide
  )?;

  Ok(())
}

fn terminal_cleanup() -> Result<()> {
  // Exit alternate screen
  execute!(std::io::stdout(),
    LeaveAlternateScreen,
    Show
  )?;

  // Disable raw mode
  terminal::disable_raw_mode()?;

  Ok(())
}