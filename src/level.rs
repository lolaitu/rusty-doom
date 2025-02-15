pub struct Level {
	layout: Vec<u8>,
	size: (u32, u32),
}

impl Level {
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
}
