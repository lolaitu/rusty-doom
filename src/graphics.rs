use std::io::Result;
use std::io::Stdout;
use crossterm::{
  cursor::MoveTo, execute, queue, style::{Color, Print, SetBackgroundColor}
};
use rayon::prelude::*;

use crate::game::Game;
use crate::weapon::WeaponSprite;
use crate::entity::{Entity, SpriteType};

#[derive(Debug, Clone)]
pub struct SpriteProjection {
    pub distance: f64,
    pub screen_x: f64,
    pub screen_y: f64,
    pub screen_width: f64,
    pub screen_height: f64,
    pub left_column: u16,
    pub right_column: u16,
    pub top_row: u16,
    pub bottom_row: u16,
    pub sprite_type: SpriteType,
}


pub struct RenderBuffer {
    pub width: u16,
    pub height: u16,
    pub buffer: Vec<Vec<(Color, char)>>,
    pub depth_buffer: Vec<f64>,
}

impl RenderBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            buffer: vec![vec![(Color::Reset, ' '); width as usize]; height as usize],
            depth_buffer: vec![0.0; width as usize],
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        if self.width != width || self.height != height {
            self.width = width;
            self.height = height;
            self.buffer = vec![vec![(Color::Reset, ' '); width as usize]; height as usize];
            self.depth_buffer = vec![0.0; width as usize];
        }
    }
    
    pub fn clear(&mut self) {
        for row in self.buffer.iter_mut() {
            for cell in row.iter_mut() {
                *cell = (Color::Reset, ' ');
            }
        }
        for depth in self.depth_buffer.iter_mut() {
            *depth = f64::MAX;
        }
    }

    pub fn set(&mut self, x: u16, y: u16, color: Color, ch: char) {
        if x < self.width && y < self.height {
            self.buffer[y as usize][x as usize] = (color, ch);
        }
    }

    pub fn flush(&self, stdout: &mut Stdout) -> Result<()> {
        queue!(stdout, MoveTo(0, 0))?;
        let mut current_color = Color::Reset;
        
        for y in 0..self.height {
            queue!(stdout, MoveTo(0, y))?;
            for x in 0..self.width {
                let (color, ch) = self.buffer[y as usize][x as usize];
                if color != current_color {
                    queue!(stdout, SetBackgroundColor(color))?;
                    current_color = color;
                }
                queue!(stdout, Print(ch))?;
            }
        }
        execute!(stdout)?;
        Ok(())
    }
}

pub fn draw(game: &Game, buffer: &mut RenderBuffer) -> Result<()>  {
  let fov = 60.0_f64;
  let ray_angle_increment = fov / game.term_size.0 as f64;
  
  // Resize buffer if needed
  buffer.resize(game.term_size.0, game.term_size.1);
  buffer.clear(); // Important to clear depth buffer

  if let Some(player) = game.world.get_player() {
    // 1. CAST RAYS & DRAW WALLS
    // Parallelize the raycasting calculation
    let column_data: Vec<(usize, f64, u8, u16, u16)> = (0..game.term_size.0)
        .into_par_iter()
        .map(|x| {
            let angle_offset = (x as f64 - game.term_size.0 as f64 / 2.0) * ray_angle_increment;
            let ray_angle = player.transform.angle + angle_offset;

            let (wall_distance, wall_type) = cast_wall_ray(
                player.transform.x, 
                player.transform.y, 
                ray_angle, 
                &game.level
            );

            let wall_height = if wall_distance > 0.1 { 
                (game.term_size.1 as f64 * 6.0) / wall_distance
            } else { 
                game.term_size.1 as f64 
            };

            let wall_start = ((game.term_size.1 as f64 - wall_height) / 2.0).max(0.0) as u16;
            let wall_end = ((game.term_size.1 as f64 + wall_height) / 2.0).min(game.term_size.1 as f64) as u16;
            
            (x as usize, wall_distance, wall_type, wall_start, wall_end)
        })
        .collect();

    // Sequential drawing to buffer (fast enough, and buffer isn't thread-safe for random access without locking)
    for (x, wall_distance, wall_type, wall_start, wall_end) in column_data {
      let x = x as u16;
      
      // Store depth for sprite occlusion
      if x < buffer.width {
          buffer.depth_buffer[x as usize] = wall_distance;
      }
      
      // Draw vertical strip
      for y in 0..game.term_size.1 {
          let color = if y < wall_start {
              Color::Rgb { r: 30, g: 50, b: 100 } // Sky
          } else if y < wall_end {
              get_wall_color(wall_distance, wall_type) // Wall
          } else {
              get_floor_color(wall_distance) // Floor
          };
          buffer.set(x, y, color, ' ');
      }
    }

    // 2. PREPARE SPRITES
    let mut sprite_projections = Vec::new();
    
    // Project all enemies
    for enemy in game.world.get_enemies() {
      if let Some(projection) = project_sprite_to_screen(
        player, enemy, game.term_size.0, game.term_size.1, fov
      ) {
        sprite_projections.push(projection);
      }
    }
    
    // Project all projectiles
    for projectile in game.world.get_projectiles() {
      if let Some(projection) = project_sprite_to_screen(
        player, projectile, game.term_size.0, game.term_size.1, fov
      ) {
        sprite_projections.push(projection);
      }
    }
    
    // Sort sprites by distance (farthest first)
    sprite_projections.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());

    // 3. DRAW SPRITES
    for sprite in sprite_projections {
        let brightness = get_distance_brightness(sprite.distance);
        let color = get_sprite_color(sprite.sprite_type, brightness);
        
        for x in sprite.left_column..=sprite.right_column {
            if x < buffer.width {
                // Z-Buffer check
                if sprite.distance < buffer.depth_buffer[x as usize] {
                    for y in sprite.top_row..=sprite.bottom_row {
                        buffer.set(x, y, color, ' ');
                    }
                }
            }
        }
    }
    
    // Draw weapon sprite overlay in bottom center
    draw_weapon_sprite(game, buffer)?;
  }
  Ok(())
}

pub fn draw_weapon_sprite(game: &Game, buffer: &mut RenderBuffer) -> Result<()> {
  let weapon_sprite = game.weapon.get_current_sprite();
  
  // Position weapon at bottom center of screen
  let start_x = (game.term_size.0 / 2).saturating_sub(weapon_sprite.width as u16 / 2);
  let start_y = game.term_size.1.saturating_sub(weapon_sprite.height as u16);

  for (line_idx, line) in weapon_sprite.lines.iter().enumerate() {
    let y = start_y + line_idx as u16;
    if y < game.term_size.1 {
      for (char_idx, character) in line.chars().enumerate() {
        if character != ' ' { // Only draw non-transparent pixels
          let color = if char_idx < weapon_sprite.colors[line_idx].len() {
            weapon_sprite.colors[line_idx][char_idx]
          } else {
            Color::Rgb { r: 100, g: 100, b: 100 }
          };
          
          buffer.set(start_x + char_idx as u16, y, color, ' ');
        }
      }
    }
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

fn get_distance_brightness(distance: f64) -> f64 {
  if distance < 1.0 { 1.0 }
  else if distance < 2.0 { 0.9 }
  else if distance < 4.0 { 0.75 }
  else if distance < 8.0 { 0.6 }
  else if distance < 12.0 { 0.4 }
  else if distance < 16.0 { 0.25 }
  else { 0.1 }
}

fn cast_wall_ray(start_x: f64, start_y: f64, angle: f64, level: &crate::level::Level) -> (f64, u8) {
  let ray_dir_x = angle.to_radians().cos();
  let ray_dir_y = angle.to_radians().sin();
  
  let mut map_x = start_x.floor() as i32;
  let mut map_y = start_y.floor() as i32;

  // Delta distance calculation (distance ray has to travel to go from 1 x-side to the next x-side)
  let delta_dist_x = if ray_dir_x == 0.0 { 1e30 } else { (1.0 / ray_dir_x).abs() };
  let delta_dist_y = if ray_dir_y == 0.0 { 1e30 } else { (1.0 / ray_dir_y).abs() };

  let mut step_x;
  let mut side_dist_x;
  
  if ray_dir_x < 0.0 {
    step_x = -1;
    side_dist_x = (start_x - map_x as f64) * delta_dist_x;
  } else {
    step_x = 1;
    side_dist_x = (map_x as f64 + 1.0 - start_x) * delta_dist_x;
  }

  let mut step_y;
  let mut side_dist_y;
  
  if ray_dir_y < 0.0 {
    step_y = -1;
    side_dist_y = (start_y - map_y as f64) * delta_dist_y;
  } else {
    step_y = 1;
    side_dist_y = (map_y as f64 + 1.0 - start_y) * delta_dist_y;
  }

  let mut hit = false;
  let mut side = 0; // 0 for NS, 1 for EW
  let mut wall_type = 0;

  // DDA Loop
  let max_steps = 50;
  let mut steps = 0;
  
  while !hit && steps < max_steps {
    steps += 1;
    // Jump to next map square, OR in x-direction, OR in y-direction
    if side_dist_x < side_dist_y {
      side_dist_x += delta_dist_x;
      map_x += step_x;
      side = 0;
    } else {
      side_dist_y += delta_dist_y;
      map_y += step_y;
      side = 1;
    }

    // Check if ray has hit a wall
    if map_y >= 0 && map_y < level.layout.len() as i32 && 
       map_x >= 0 && map_x < level.layout[0].len() as i32 {
      let w = level.layout[map_y as usize][map_x as usize];
      if w > 0 {
        hit = true;
        wall_type = w;
      }
    } else {
      // Hit boundary
      hit = true;
      wall_type = 1;
    }
  }

  // Calculate distance projected on camera direction (Euclidean distance would give fisheye effect!)
  let perp_wall_dist = if side == 0 {
    (side_dist_x - delta_dist_x)
  } else {
    (side_dist_y - delta_dist_y)
  };

  (perp_wall_dist, wall_type)
}


fn project_sprite_to_screen(
  player: &Entity, 
  sprite_entity: &Entity, 
  screen_width: u16, 
  screen_height: u16, 
  fov: f64
) -> Option<SpriteProjection> {
  // Calculate distance and angle from player to sprite
  let dx = sprite_entity.transform.x - player.transform.x;
  let dy = sprite_entity.transform.y - player.transform.y;
  let distance = (dx * dx + dy * dy).sqrt();
  
  if distance < 0.1 { return None; } // Too close
  
  // Calculate angle from player to sprite
  let sprite_angle = dy.atan2(dx).to_degrees();
  let mut relative_angle = sprite_angle - player.transform.angle;
  
  // Normalize angle to [-180, 180]
  while relative_angle > 180.0 { relative_angle -= 360.0; }
  while relative_angle < -180.0 { relative_angle += 360.0; }
  
  // Check if sprite is within FOV
  let half_fov = fov / 2.0;
  if relative_angle.abs() > half_fov { return None; }
  
  // Calculate screen x position (0.0 = left edge, 1.0 = right edge)
  let screen_x_normalized = (relative_angle + half_fov) / fov;
  let screen_x = screen_x_normalized * screen_width as f64;
  
  // Calculate sprite size based on distance
  let base_sprite_size = 1.0; // World units
  let projected_height = (base_sprite_size * screen_height as f64) / distance;
  let projected_width = projected_height * 0.8; // Sprites are slightly narrower than tall
  
  // Calculate screen bounds
  let screen_y = screen_height as f64 / 2.0; // Center vertically
  let left = (screen_x - projected_width / 2.0).max(0.0);
  let right = (screen_x + projected_width / 2.0).min(screen_width as f64);
  let top = (screen_y - projected_height / 2.0).max(0.0);
  let bottom = (screen_y + projected_height / 2.0).min(screen_height as f64);
  
  // Skip if sprite is entirely off screen
  if left >= right || top >= bottom { return None; }
  
  Some(SpriteProjection {
    distance,
    screen_x,
    screen_y,
    screen_width: projected_width,
    screen_height: projected_height,
    left_column: left as u16,
    right_column: right as u16,
    top_row: top as u16,
    bottom_row: bottom as u16,
    sprite_type: sprite_entity.sprite_type,
  })
}



fn get_sprite_color(sprite_type: SpriteType, brightness: f64) -> Color {
  match sprite_type {
    SpriteType::EnemyImp => darken_color(200, 100, 50, brightness), // Orange/brown imp
    SpriteType::EnemyDemon => darken_color(150, 50, 50, brightness), // Dark red demon
    SpriteType::None => darken_color(255, 200, 0, brightness), // Yellow projectile
  }
}