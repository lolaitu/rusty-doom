// Écriture et gestion des erreur
use std::io::{stdout, Write, Result};

// gestion des appelles sur le terminal
use crossterm::{
    queue,
    execute,
    terminal::{self,
        EnterAlternateScreen,
        LeaveAlternateScreen
    },
    event::{self, Event, KeyCode},
    cursor::{Hide, Show, MoveTo}
};

use std::fmt;               // print sur objet
use std::thread::sleep;     // sleep
use std::time::Duration;    // gestion des temps
use rand::Rng;              // gestion aléatoire
//use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};
//use enigo::{Enigo, MouseControllable}; // To move the mouse

mod game;
use game::Game;

mod graphics;

mod level;

mod player;  // Importer le fichier player.rs
//use player::Player;  // Importer la struct Player

mod mainGame;
//use mainGame::MainGame;



fn main() -> Result<()>{

    //terminal_init();

    let mut test = level::Level::debug_1()?;
    test.print();

    //let MAZE: &Vec<Vec<char>> = &maze;

    // let mut mainGame = MainGame::new(&test);
    // mainGame.init()?;

    //let mut my_game = Game::new()?;
    //my_game.launch()?;

    //terminal_cleanup();

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
