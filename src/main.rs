mod game;
mod graphics;
mod level;

use std::io::Result;
use crossterm::{
    execute,
    cursor::{Hide, Show},
    terminal::{self,
        EnterAlternateScreen,
        LeaveAlternateScreen
    },
};
use game::Game;


fn main() -> Result<()> {
    let mut write = std::io::stdout();

    terminal::enable_raw_mode()?;
    execute!(write,
        EnterAlternateScreen,
        Hide
    )?;

    let mut test = level::Level::new()?;

    let mut my_game = Game::new()?;
    my_game.launch()?;

    execute!(write,
        LeaveAlternateScreen,
        Show
    )?;
    terminal::disable_raw_mode()?;

    Ok(())
}
