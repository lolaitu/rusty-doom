/*
use std::io::{stdout, Write};
use std::io::Result;
use device_query::{DeviceQuery, DeviceState, MouseState, Keycode};
use enigo::{Enigo, MouseControllable};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
    style::{Color, Print, SetBackgroundColor, SetForegroundColor, ResetColor},
    cursor::MoveTo,};
use crossterm::style::Stylize;
use std::thread::sleep;
use std::time::Duration;
use crate::Player;



pub struct MainGame<'a> {
    pub map: &'a Vec<Vec<char>>, // Game map represented in a vector
    pub stop: bool, // To know if game is running
    pub stdout: std::io::Stdout, // Used to show text in terminal
    pub device_state: DeviceState,
    pub enigo: Enigo, // For moving the mouse
    pub player: Player, // Player is in this var
    pub screen_size: (usize, usize),
    pub keys: Vec<Keycode>,
}

impl<'a> MainGame<'a> {
    // Constructeur
    pub fn new(map: &'a Vec<Vec<char>>) -> MainGame {
        MainGame {
            map: map,
            stop: false,
            stdout: stdout(),
            device_state: DeviceState::new(),
            enigo: Enigo::new(),
            player: Player::new(80.0, 25.0, 0.0, (720, 0)),
            screen_size: (500,120),
            keys: Vec::new(),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        execute!(self.stdout, terminal::Clear(ClearType::All))?;

        self.play()?;

        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn play(&mut self) -> Result<()> {
        self.enigo.mouse_move_to(720, 900);
        let mut prev_mouse_x_pos: i32 = 720;

        while !self.stop {
            let mouse: MouseState = self.device_state.get_mouse(); // get the current position of the mouse
            let mut delta_mouse = mouse.coords.0 - prev_mouse_x_pos; //compare previous position of the mouse to the current one
            self.keys = self.device_state.get_keys(); // stores all the key pressed

            if mouse.coords.0 == 0 || mouse.coords.0 == 1439{ // if we touch the border of the screen
                self.enigo.mouse_move_to(720, 900);
                prev_mouse_x_pos = 720;
            }
            else{
                prev_mouse_x_pos = mouse.coords.0;
            }
            if mouse.coords.1 != 900{ // keep the mouse outside the screen
                self.enigo.mouse_move_to(mouse.coords.0, 900);
            }

            if !self.keys.is_empty() {
                if self.keys.contains(&Keycode::Escape) { // to stop the program
                    self.stop = true;
                }
            }

            // Render the game in terminal
            self.update(&mouse, delta_mouse);
            self.ray_render()?;
            //self.render()?;
            // Pause of 100 ms to limit fresh rate
            std::thread::sleep(std::time::Duration::from_millis(100));

        }

        Ok(())
    }

    pub fn update(&mut self, mouse: &MouseState, delta_mouse: i32) -> bool {
        let position = self.player.position;
        let nw = self.map[position.1 as usize][position.0 as usize];
        let n = self.map[position.1 as usize][position.0 as usize + 1];
        let ne = self.map[position.1 as usize][position.0 as usize + 2];
        let sw = self.map[position.1 as usize + 2][position.0 as usize];
        let s = self.map[position.1 as usize + 2][position.0 as usize + 1];
        let se = self.map[position.1 as usize + 2][position.0 as usize + 2];
        let e = self.map[position.1 as usize + 1][position.0 as usize + 2];
        let w = self.map[position.1 as usize + 1][position.0 as usize];

        let environement = [nw, n, ne, w, e, sw, s, se];
        self.player.move_player(delta_mouse as f64, environement, &self.keys);

        true
    }

    pub fn render(&mut self) -> Result<()>{
        execute!(self.stdout, MoveTo(0, 0))?;

        for (y, line) in self.map.iter().enumerate() {
            execute!(self.stdout, MoveTo(0, y as u16))?;

            for (x, &ch) in line.iter().enumerate() {
                if x == ((self.player.position.0 as i32)+1) as usize && y == ((self.player.position.1 as i32)+1) as usize{
                    print!("O");
                }
                else{
                    print!("{}", ch);
                }
            }
            println!();
        }
        //println!("L'angle : {}", self.player.angle);
        self.stdout.flush()?;
        Ok(())
    }

    pub fn ray_render(&mut self) -> Result<()>{
        execute!(self.stdout, MoveTo(0, 0))?;

        let fov  = 60.0; // field of view in degrees
        let ray_angle_increment: f64 = fov as f64 / self.screen_size.0 as f64;

        for x in 0..self.screen_size.0 {
            // Calculate ray angle (player angle +/- half FOV)
            let ray_angle = self.player.angle + 2.0 * std::f64::consts::PI - (fov / 2.0) + (x as f64 * ray_angle_increment);

            // Calculate ray direction vectors
            let ray_dir_x = ray_angle.to_radians().cos();
            let ray_dir_y = ray_angle.to_radians().sin();

            // Ray starting position (player position)
            let mut ray_x = self.player.position.0;
            let mut ray_y = self.player.position.1;

            // Cast ray until we hit a wall
            let mut distance = 0.0;
            let step_size = 0.1; // Smaller step size for more precise detection

            while distance < 120.0 { // Maximum view distance
                ray_x += ray_dir_x * step_size;
                ray_y += ray_dir_y * step_size;
                distance += step_size;

                // Check if ray hit a wall
                let map_x = ray_x as usize;
                let map_y = ray_y as usize;

                if map_y < self.map.len() && map_x < self.map[map_y].len() {
                    if self.map[map_y][map_x] == '#' {
                        // Calculate wall height based on distance
                        let wall_height = (self.screen_size.1 as f64 / distance) * 20.0;
                        let wall_start = ((self.screen_size.1 as f64 - wall_height) / 2.0).max(0.0) as usize;
                        let wall_end = ((self.screen_size.1 as f64 + wall_height) / 2.0).min(self.screen_size.1 as f64) as usize;

                        // Draw wall column
                        for y in 0..self.screen_size.1 {
                            execute!(self.stdout, MoveTo(x as u16, y as u16))?;
                            if y < wall_start {
                                print!(" "); // Sky
                            } else if y < wall_end {
                                // Wall shading based on distance
                                let shade = if distance < 5.0 { "█" }
                                          else if distance < 10.0 { "▓" }
                                          else if distance < 15.0 { "▒" }
                                          else { "░" };
                                let color = (255.0 * (10.0/distance)) as u8;
                                //let color = 255;
                                print!("{}", " ".on(Color::Rgb { r: color, g: color, b: color }));
                            } else {
                                print!("."); // Floor
                            }
                        }
                        break;
                    }
                }
            }
        }

        // for (y, line) in self.map.iter().enumerate() {
        //     execute!(self.stdout, MoveTo(0, y as u16))?;

        //     for (x, &ch) in line.iter().enumerate() {
        //         if x == ((self.player.position.0 as i32)+1) as usize && y == ((self.player.position.1 as i32)+1) as usize{
        //             print!("O");
        //         }
        //         else{
        //             print!("{}", ch);
        //         }
        //     }
        //     println!();
        // }
        //println!("L'angle : {}", self.player.angle);
        self.stdout.flush()?;
        Ok(())
    }
}
*/
