use std::error::Error;
use crossterm::{
    queue,
    cursor::MoveTo,
    style::Print,
};

pub struct Level {
	layout: Vec<u8>,
	size: (u32, u32),
}

impl Level {

	pub fn print(&self) {
        let (width, height) = self.size;
        for y in 0..height {
            for x in 0..width {
                let cell = self.layout[(y * width + x) as usize];
                let symbol = if cell == 1 { '█' } else { ' ' };
                queue!(std::io::stdout(),
                	MoveTo(x as u16, y as u16),
                	Print(symbol),
                );
            }
        }
        //std::io::stdout().flush()?;
    }

	pub fn new() -> Result<Self, std::io::Error> {

		Ok( Self{
			layout: vec![
				1, 1, 1, 1, 1,
				1, 0, 0, 0, 1,
				1, 0, 0, 0, 1,
				1, 0, 0, 0, 1,
				1, 1, 1, 1, 1,
			],
			size: (5, 5)
		})
	}

	pub fn debug_1() -> Result<Self, std::io::Error> {
		let mut layout = Vec::new();
        let mut rng = rand::thread_rng();

        let width = 160;
        let height = 40;

        for y in 0..height {
            for x in 0..width {
                if y == 0 || y == height - 1 || x == 0 || x == width - 1 {
                    layout.push(1); // Bordures
                } else if (y == 20 && x > 50 && x < 100) || (x == 50 && y > 20 && y < 30) {
                    layout.push(1); // Mur
                } else {
                    layout.push(0); // Espace
                }
            }
        }
	    Ok(Self {
	        layout,
	        size: (width, height),
	    })
	}


}
