use std::io::Result;
use crossterm::{
    queue,
    cursor::{MoveTo},
    style::{Print, SetBackgroundColor, Color},
    terminal,
};

use crate::game::Game;


pub fn draw(game: &Game) -> Result<()> {
	let mut write = std::io::stdout();
	let term_size = terminal::size()?;

	for j in 0..term_size.1 {
	    for i in 0..term_size.0 {
	        let uv = {(
	            i as f32 / term_size.0 as f32,
	            j as f32 / term_size.1 as f32
	        )};

	        queue!(write,
	            MoveTo(i, j),
	            SetBackgroundColor(Color::Rgb{
	                r: (uv.0 * 255.0) as u8,
	                g: (uv.1 * 255.0) as u8,
	                b: ( ((game.time_of_launch.elapsed().as_millis() as f32 / 1000_f32).sin() + 1_f32) * 128_f32 ) as u8
	                //b: ((self.time_of_launch.elapsed().as_millis() as f32 / 10_f32) as u32 % 255) as u8
	            }),
	            Print(' '),
	        )?;
	    }
	}

	Ok(())
}
