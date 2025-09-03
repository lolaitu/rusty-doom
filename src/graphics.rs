use std::io::Result;
use std::io::Stdout;
use crossterm::{
  cursor::MoveTo, execute, queue, style::{Color, Print, SetBackgroundColor}
};

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


pub fn draw(game: &Game, stdout: &mut Stdout) -> Result<()>  {
  let fov = 60.0_f64;
  let ray_angle_increment = fov / game.term_size.0 as f64;

  if let Some(player) = game.world.get_player() {
    // Pre-calculate sprite projections
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
    
    // Sort sprites by distance (farthest first for proper depth sorting)
    sprite_projections.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());

    // Pre-calculate all wall rays
    let mut column_data = Vec::with_capacity(game.term_size.0 as usize);
    
    for x in 0..game.term_size.0 {
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
      
      column_data.push((wall_distance, wall_type, wall_start, wall_end));
    }

    // Draw row by row for better terminal performance
    for y in 0..game.term_size.1 {
      queue!(stdout, MoveTo(0, y))?;
      for x in 0..game.term_size.0 {
        let (wall_distance, wall_type, wall_start, wall_end) = column_data[x as usize];
        
        // Check if there's a sprite at this pixel position
        let sprite_color = get_sprite_color_at_pixel(x, y, &sprite_projections, wall_distance);
        
        let color = if let Some(sprite_col) = sprite_color {
          sprite_col // Draw sprite (already depth tested)
        } else if y < wall_start {
          Color::Rgb { r: 30, g: 50, b: 100 } // Sky
        } else if y < wall_end {
          get_wall_color(wall_distance, wall_type) // Wall
        } else {
          get_floor_color(wall_distance) // Floor
        };

        queue!(stdout, SetBackgroundColor(color), Print(" "))?;
      }
    }
    
    // Draw weapon sprite overlay in bottom center
    draw_weapon_sprite(game, stdout)?;
    
    execute!(stdout)?;
  }
  Ok(())
}

pub fn draw_weapon_sprite(game: &Game, stdout: &mut Stdout) -> Result<()> {
  let weapon_sprite = game.weapon.get_current_sprite();
  
  // Position weapon at bottom center of screen
  let start_x = (game.term_size.0 / 2).saturating_sub(weapon_sprite.width as u16 / 2);
  let start_y = game.term_size.1.saturating_sub(weapon_sprite.height as u16);

  for (line_idx, line) in weapon_sprite.lines.iter().enumerate() {
    let y = start_y + line_idx as u16;
    if y < game.term_size.1 {
      queue!(stdout, MoveTo(start_x, y))?;
      
      for (char_idx, character) in line.chars().enumerate() {
        if character != ' ' { // Only draw non-transparent pixels
          let color = if char_idx < weapon_sprite.colors[line_idx].len() {
            weapon_sprite.colors[line_idx][char_idx]
          } else {
            Color::Rgb { r: 100, g: 100, b: 100 }
          };
          
          queue!(stdout, SetBackgroundColor(color), Print(" "))?;
        } else {
          // Transparent pixel - move cursor forward
          queue!(stdout, MoveTo(start_x + char_idx as u16 + 1, y))?;
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

fn get_sprite_color_at_pixel(
  x: u16, 
  y: u16, 
  sprite_projections: &[SpriteProjection], 
  wall_distance: f64
) -> Option<Color> {
  // Check all sprites from nearest to farthest (already sorted)
  for projection in sprite_projections {
    // Check if this pixel is within sprite bounds
    if x >= projection.left_column && x <= projection.right_column &&
       y >= projection.top_row && y <= projection.bottom_row {
      
      // Depth test: only render if sprite is closer than wall
      if projection.distance < wall_distance {
        let brightness = get_distance_brightness(projection.distance);
        return Some(get_sprite_color(projection.sprite_type, brightness));
      }
    }
  }
  None
}

fn get_sprite_color(sprite_type: SpriteType, brightness: f64) -> Color {
  match sprite_type {
    SpriteType::EnemyImp => darken_color(200, 100, 50, brightness), // Orange/brown imp
    SpriteType::EnemyDemon => darken_color(150, 50, 50, brightness), // Dark red demon
    SpriteType::None => darken_color(255, 200, 0, brightness), // Yellow projectile
  }
}