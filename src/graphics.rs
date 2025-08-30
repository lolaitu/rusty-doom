use std::io::Result;
use std::io::Stdout;
use crossterm::{
  cursor::MoveTo, execute, queue, style::{Color, Print, SetBackgroundColor}
};

use crate::game::Game;


pub fn draw(game: &Game, stdout: &mut Stdout) -> Result<()>  {
  let fov = 60.0_f64;
  let ray_angle_increment = fov / game.term_size.0 as f64;

  if let Some(player) = game.world.get_player() {
    // Pre-calculate all rays to avoid redundant calculations
    let mut column_data = Vec::with_capacity(game.term_size.0 as usize);
    
    for x in 0..game.term_size.0 {
      let angle_offset = (x as f64 - game.term_size.0 as f64 / 2.0) * ray_angle_increment;
      let ray_angle = player.transform.angle + angle_offset;

      let (distance, wall_type) = cast_ray(
        player.transform.x, 
        player.transform.y, 
        ray_angle, 
        &game.level
      );

      let wall_height = if distance > 0.1 { 
        (game.term_size.1 as f64 * 6.0) / distance
      } else { 
        game.term_size.1 as f64 
      };

      let wall_start = ((game.term_size.1 as f64 - wall_height) / 2.0).max(0.0) as u16;
      let wall_end = ((game.term_size.1 as f64 + wall_height) / 2.0).min(game.term_size.1 as f64) as u16;
      
      column_data.push((distance, wall_type, wall_start, wall_end));
    }

    // Draw row by row for better terminal performance
    for y in 0..game.term_size.1 {
      queue!(stdout, MoveTo(0, y))?;
      for x in 0..game.term_size.0 {
        let (distance, wall_type, wall_start, wall_end) = column_data[x as usize];
        
        let color = if y < wall_start {
          Color::Rgb { r: 30, g: 50, b: 100 } // Sky
        } else if y < wall_end {
          get_wall_color(distance, wall_type) // Wall
        } else {
          get_floor_color(distance) // Floor
        };

        queue!(stdout, SetBackgroundColor(color), Print(" "))?;
      }
    }
    execute!(stdout)?;
  }
  Ok(())
}

fn darken_color(r: u8, g: u8, b: u8, factor: f64) -> Color {
  let new_r = ((r as f64) * factor).min(255.0) as u8;
  let new_g = ((g as f64) * factor).min(255.0) as u8; 
  let new_b = ((b as f64) * factor).min(255.0) as u8;
  Color::Rgb { r: new_r, g: new_g, b: new_b }
}

fn get_wall_color(distance: f64, wall_type: u8) -> Color {
  // Base RGB colors for each wall type
  let (r, g, b) = match wall_type {
    1 => (200, 200, 200), // Light grey stone
    2 => (180, 60, 60),   // Red brick
    3 => (200, 180, 60),  // Yellow stone
    4 => (60, 100, 180),  // Blue stone
    5 => (160, 60, 160),  // Purple crystal
    _ => (120, 120, 120), // Default grey
  };

  // Distance-based darkening - smoother gradients
  let brightness = if distance < 1.0 { 1.0 }
                  else if distance < 15.0 { 1.06 - 0.9 * distance / 15.0 }
                  else { 1.1 - 0.9 * distance / 15.0 };
  /*                else if distance < 2.0 { 0.85 }
                  else if distance < 3.0 { 0.7 }
                  else if distance < 5.0 { 0.55 }
                  else if distance < 8.0 { 0.4 }
                  else if distance < 12.0 { 0.25 }
                  else if distance < 16.0 { 0.1 }
                  else { 0.05 };*/

  darken_color(r, g, b, brightness)
}

fn get_floor_color(distance: f64) -> Color {
  // Green floor that darkens with distance
  let (r, g, b) = (40, 80, 40); // Dark green base
  
  let brightness = if distance < 2.0 { 0.8 }
                  else if distance < 4.0 { 0.6 }
                  else if distance < 8.0 { 0.4 }
                  else if distance < 12.0 { 0.2 }
                  else { 0.05 };

  darken_color(r, g, b, brightness)
}

fn cast_ray(start_x: f64, start_y: f64, angle: f64, level: &crate::level::Level) -> (f64, u8) {
  let ray_dir_x = angle.to_radians().cos();
  let ray_dir_y = angle.to_radians().sin();
  
  let mut ray_x = start_x;
  let mut ray_y = start_y;
  let step_size = 0.05;
  let mut distance = 0.0;

  while distance < 20.0 {
    ray_x += ray_dir_x * step_size;
    ray_y += ray_dir_y * step_size;
    distance += step_size;

    let map_x = ray_x as usize;
    let map_y = ray_y as usize;
    
    if map_y < level.layout.len() && map_x < level.layout[0].len() {
      let wall_type = level.layout[map_y][map_x];
      if wall_type > 0 {
        return (distance, wall_type);
      }
    } else {
      return (distance, 1); // Hit boundary
    }
  }
  
  (20.0, 0) // Max distance, no wall
}