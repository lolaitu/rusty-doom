use crossterm::{
	queue,
	execute,
	cursor::MoveTo,
	style::{Print, Color, SetBackgroundColor},
};
  
use crate::entity::Entity;
  
pub struct Level {
	pub layout: Vec<Vec<u8>>,
	pub size: (u32, u32),
}
  
impl Level {
	pub fn print(&self) -> Result<(), std::io::Error> {
		let (width, height) = self.size;
		for y in 0..height {
			for x in 0..width {
				let cell = self.layout[y as usize][x as usize];
				let symbol = if cell != 0 { '█' } else { ' ' };
				queue!(std::io::stdout(),
					MoveTo(x as u16, y as u16),
					Print(symbol),
				)?;
			}
		}
		execute!(std::io::stdout())?;
		Ok(())
	}
  
	pub fn print_with_player_entity(&self, player: &Entity) -> Result<(), std::io::Error> {
		let (width, height) = self.size;
		let (player_x, player_y) = (player.transform.x as u32, player.transform.y as u32);
		for y in 0..height {
			for x in 0..width {
				let cell = self.layout[y as usize][x as usize];
				let color = if cell != 0 { Color::White } else { Color::Black };
				let symbol = if x == player_x && y == player_y {
					if      45.  < player.transform.angle && player.transform.angle <= 135. {"°°"}
					else if 135. < player.transform.angle && player.transform.angle <= 225. {":-"}
					else if 225. < player.transform.angle && player.transform.angle <= 315. {".."}
					else                                                {"-:"}
				} else {"  "};
				queue!(std::io::stdout(),
					MoveTo(2 * x as u16, y as u16),
					SetBackgroundColor(color),
					Print(symbol),
				)?;
			}
		}
		execute!(std::io::stdout())?;
		Ok(())
	}
  
	pub fn debug_1() -> Result<Self, std::io::Error> {
		let layout = vec![
		vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
		vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
		vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
		vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
		vec![1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
		vec![1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
		vec![1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
		vec![1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
		vec![1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
		vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
		vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
		vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
		vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
		vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
		vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
		vec![1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
		vec![1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
		vec![1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
		vec![1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
		vec![1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
		vec![1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
		vec![1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
	    vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
	];
  
	Ok(Self {
		layout: layout,
		size: (24, 24),
	})
	}
}