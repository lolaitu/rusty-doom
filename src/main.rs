use std::io::{stdout, Write, Result};
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use device_query::{DeviceQuery, DeviceState, MouseState, Keycode}; // To detect keyboard and mouse position
use enigo::{Enigo, MouseControllable}; // To move the mouse
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
    cursor::MoveTo,
};

mod game;
use game::Game;

mod graphics;

mod level;

mod player;  // Importer le fichier player.rs
use player::Player;  // Importer la struct Player

mod mainGame;
use mainGame::MainGame;



fn main() -> Result<()>{

    terminal_init()

    let mut test = level::Level::debug_1()?;

    //let MAZE: &Vec<Vec<char>> = &maze;

    // let mut mainGame = MainGame::new(&test);
    // mainGame.init()?;

    //let mut my_game = Game::new()?;
    //my_game.launch()?;

    terminal_cleanup()

    Ok(())
}

fn terminal_init() -> Result<()> {

    terminal::enable_raw_mode()?;

    execute!(std::io::stdout(),
        EnterAlternateScreen,
        Hide
    )?;
}

fn terminal_cleanup() -> Result<()> {

    execute!(std::io::stdout(),
        LeaveAlternateScreen,
        Show
    )?;

    terminal::disable_raw_mode()?;
}
