// player.rs
//use device_query::{DeviceQuery, DeviceState, MouseState, Keycode}; // To detect keyboard and mouse position
use device_query::{DeviceQuery, DeviceState, Keycode}; // To detect keyboard and mouse position

pub struct Player {
    pub position: (f64, f64),  // Position du joueur sous forme de coordonnées (x, y)
    pub direction: f64, 
    pub x_mouse: i32,
    pub y_mouse: i32,
    pub angle: f64,
          // Angle de direction en radians ou degrés
}

impl Player {
    // Constructeur
    pub fn new(x: f64, y: f64, direction: f64, cursor: (i32,i32)) -> Player {
        Player {
            position: (x, y),
            direction,
            x_mouse: cursor.0,
            y_mouse: cursor.1,
            angle: 0.0,
        }
    }

    // Méthode pour déplacer le joueur
    pub fn move_player(&mut self, delta_mouse: f64, environement: [char;8], keys: &Vec<Keycode>) {
        // Update angle based on cursor position
        let mouse_sensitivity = 0.01;
        self.angle += delta_mouse * mouse_sensitivity;

        let device_state = DeviceState::new();
        
        if !keys.is_empty() {
            let current_pos = (self.position.0 as usize, self.position.1 as usize);
            let mut next_pos = (self.position.0, self.position.1);
            let movement_speed = 0.5; // Add a speed constant to control movement

            // Calculate movement vector
            let mut dx = 0.0;
            let mut dy = 0.0;
            
            if keys.contains(&Keycode::W) {
                dx -= self.angle.cos();
                dy -= self.angle.sin();
            }
            if keys.contains(&Keycode::S) {
                dx += self.angle.cos();
                dy += self.angle.sin();
            }
            if keys.contains(&Keycode::A) {
                dx -= self.angle.sin();
                dy += self.angle.cos();
            }
            if keys.contains(&Keycode::D) {
                dx += self.angle.sin();
                dy -= self.angle.cos();
            }

            // Normalize the movement vector if it's longer than 1
            let length = (dx * dx + dy * dy).sqrt();
            if length > 1.0 {
                dx /= length;
                dy /= length;
            }

            // Apply movement speed after normalization
            dx *= movement_speed;
            dy *= movement_speed;

            // Check movement in smaller steps
            let steps = 4; // Divide movement into smaller steps
            for i in 0..steps {
                let step_dx = dx / steps as f64;
                let step_dy = dy / steps as f64;

                // Try X movement
                next_pos.0 = self.position.0 + step_dx;
                if !self.check_collision((next_pos.0 as usize, self.position.1 as usize), &environement) {
                    self.position.0 = next_pos.0;
                }

                // Try Y movement
                next_pos.1 = self.position.1 + step_dy;
                if !self.check_collision((self.position.0 as usize, next_pos.1 as usize), &environement) {
                    self.position.1 = next_pos.1;
                }
            }
        }
    }

    pub fn check_collision(&mut self, pos: (usize, usize), environement: &[char;8]) -> bool{
        // Check if there's a wall surrounding the player
        // [0][1][2]
        // [3][P][4]
        // [5][6][7]

        let x_diff = pos.0 as i32 - self.position.0 as i32;
        let y_diff = pos.1 as i32 - self.position.1 as i32;
    
        match (x_diff, y_diff) {
            (-1, -1) => environement[0] == '#', // Northwest
            (0, -1) => environement[1] == '#',  // North
            (1, -1) => environement[2] == '#',  // Northeast
            (-1, 0) => environement[3] == '#',  // West
            (1, 0) => environement[4] == '#',   // East
            (-1, 1) => environement[5] == '#',  // Southwest
            (0, 1) => environement[6] == '#',   // South
            (1, 1) => environement[7] == '#',   // Southeast
            _ => false
        }
    }

}

