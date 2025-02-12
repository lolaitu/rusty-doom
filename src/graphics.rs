use std::io::{Stdout, Write, Result};
use std::time::{Duration, Instant};
use crossterm::{
    queue,
    cursor::{MoveTo},
    event::{self, Event, KeyCode, KeyModifiers},
    style::{Print, SetBackgroundColor, Color},
    terminal,
};
//type write = std::io::stdout();

use crate::game::Game;

pub fn draw(state: &mut Game) -> Result<()> {
	queue!(state.write,
		Print(' ')
	)?;
	Ok(())
}
