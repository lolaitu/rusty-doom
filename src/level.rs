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

	pub fn debug_1() -> Result<Self, std::io::Error> {
		let mut maze = Vec::new();
	    let mut rng = rand::thread_rng();

	    for y in 0..40 {
	        if y == 0 || y == 39 {
	            // Bord supérieur et inférieur rempli de #
	            maze.push(vec!['#'; 160]);
	        } else {
	            // Bordures gauche et droite avec des espaces au milieu et des # aléatoires
	            let mut line = vec!['#'];
	            for x in 1..159 {
	                // 5% chance to place a wall
	                //if rng.gen_ratio(1, 20) {
	                //    line.push('#');
	                //} else {

	                //}
	                if (y == 20 && x<100 && x>50) || (x == 50 && y>20 && y<30){
	                    line.push('#');
	                }
	                else{
	                    line.push(' ');
	                }
	            }
	            line.push('#');
	            maze.push(line);
	        }
	    }
	maze
	}
}
