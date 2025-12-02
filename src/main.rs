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

mod modes;


pub mod graphics;
pub mod network;


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

use modes::{SoloGame, HostGame, ClientGame, GameMode};

// Main program loop
fn main() -> Result<()>{
  // Initialize terminal
  terminal_init()?;

  // Load level
  let level = level::Level::debug_1()?;
  
  // Parse arguments
  let args: Vec<String> = std::env::args().collect();
  let mut game: Box<dyn GameMode> = if args.contains(&"--host".to_string()) {
      Box::new(HostGame::new(level)?)
  } else if args.contains(&"--client".to_string()) {
      Box::new(ClientGame::new()?)
  } else {
      Box::new(SoloGame::new(level)?)
  };

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
    if game.update(&input_manager)? {
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