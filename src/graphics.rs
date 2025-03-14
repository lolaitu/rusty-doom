use std::io::Result;
use std::io::Stdout;
use crossterm::{
  cursor::MoveTo, execute, queue, style::{Color, Print, SetBackgroundColor}, terminal
};

use crate::game::Game;


pub fn draw(game: &Game, stdout: &mut Stdout) -> Result<()>  {
  let fov  = 60.0; // field of view in degrees
  let ray_angle_increment: f64 = fov as f64 / game.term_size.0 as f64;

  for x in 0..game.term_size.0 {
    // Calculate ray angle (player angle +/- half FOV)
    let ray_angle = game.joueur.angle + 2.0 * std::f64::consts::PI - (fov / 2.0) + (x as f64 * ray_angle_increment);

    // Calculate ray direction vectors
    let ray_dir_x = ray_angle.to_radians().cos();
    let ray_dir_y = ray_angle.to_radians().sin();
    // Ray starting position (player position)
    let mut ray_x = game.joueur.x;
    let mut ray_y = game.joueur.y;

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
      
      if map_y < game.level.size.0 as usize && map_x < game.level.size.1 as usize {
        if game.level.layout[map_y][map_x] > 0 {
          // Calculate wall height based on distance
          let wall_height = (game.level.size.0 as f64 / distance) * 20.0;
          let wall_start = ((game.level.size.1 as f64 - wall_height) / 2.0).max(0.0) as u16;
          let wall_end = ((game.level.size.1 as f64 + wall_height) / 2.0).min(game.term_size.1 as f64) as usize;


          // Draw wall column
          for y in 0..game.term_size.1 {
            execute!(stdout, MoveTo(x as u16, y as u16))?;
            if y < wall_start {
              print!("*"); // Sky
            } else if y < wall_end as u16 {
              // Wall shading based on distance
              let shade = if distance < 5.0 { print!("a") }
                          else if distance < 10.0 { print!("b") }
                          else if distance < 15.0 { print!("c") }
                          else { print!("d") };
            } else {
              print!("."); // Floor
            }
          }                    
          break;
        }
      }
    }
  }
  Ok(())
}
