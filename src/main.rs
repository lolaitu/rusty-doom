// Écriture et gestion des erreur
use std::io::{stdout, Write, Result};

// gestion des appelles sur le terminal
use crossterm::{
    queue,
    execute,
    style::Print,
    cursor::{Hide, Show, MoveTo},
    terminal::{self,
        EnterAlternateScreen,
        LeaveAlternateScreen
    },
    event::{self, Event, KeyCode, KeyModifiers},
};

use std::time::Duration;    // gestion des temps

mod game;
use game::Game;

mod graphics;

mod level;

mod player;  // Importer le fichier player.rs

mod joueur;



fn main() -> Result<()>{

    terminal_init()?;

    let test = level::Level::debug_1()?;

    let mut my_game = Game::new(test)?;
    my_game.launch()?;

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

fn wait_ctrl_c() -> Result<()> {
    loop {
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
