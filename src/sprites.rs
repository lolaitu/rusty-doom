use crossterm::style::Color;
use crate::entity::{SpriteType, EntityState};

#[derive(Clone)]
pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Option<Color>>, // None for transparent
}

impl Sprite {
    pub fn new(width: usize, height: usize, pixels: Vec<Option<Color>>) -> Self {
        Self { width, height, pixels }
    }

    pub fn get_pixel(&self, u: f64, v: f64) -> Option<Color> {
        let x = (u * self.width as f64).floor() as usize;
        let y = (v * self.height as f64).floor() as usize;
        
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x]
        } else {
            None
        }
    }
}

pub fn get_sprite_frame(sprite_type: SpriteType, frame: usize, state: EntityState) -> Sprite {
    let mut sprite = match sprite_type {
        SpriteType::EnemyImp => {
            let frames = [create_imp_sprite_frame1(), create_imp_sprite_frame2()];
            frames[frame % 2].clone()
        },
        SpriteType::EnemyDemon => {
            let frames = [create_demon_sprite_frame1(), create_demon_sprite_frame2()];
            frames[frame % 2].clone()
        },
        SpriteType::None => create_projectile_sprite(),
        SpriteType::ProjectilePistol => create_projectile_pistol(),
        SpriteType::ProjectileShotgun => create_projectile_shotgun(),
        SpriteType::ProjectileGatling => create_projectile_gatling(),
    };

    // Apply state effects
    match state {
        EntityState::Hit => {
            // Flash white/bright
            for pixel in sprite.pixels.iter_mut() {
                if let Some(color) = pixel {
                    *color = Color::Rgb { r: 255, g: 255, b: 255 };
                }
            }
        },
        EntityState::Dying => {
            // Darken or turn red for death
            for pixel in sprite.pixels.iter_mut() {
                if let Some(color) = pixel {
                    match color {
                        Color::Rgb { r, g, b } => {
                            *color = Color::Rgb { r: *r / 2, g: *g / 2, b: *b / 2 };
                        }
                        _ => {}
                    }
                }
            }
        },
        _ => {}
    }

    sprite
}

pub fn get_animation_duration(sprite_type: SpriteType) -> f64 {
    match sprite_type {
        SpriteType::EnemyImp => 0.5, // 0.5 seconds per frame
        SpriteType::EnemyDemon => 0.4,
        _ => 1.0,
    }
}

fn create_imp_sprite_frame1() -> Sprite {
    // 8x8 Imp Sprite (Frame 1 - Arms down)
    let width = 8;
    let height = 8;
    let mut pixels = vec![None; width * height];
    
    let c1 = Some(Color::Rgb { r: 139, g: 69, b: 19 }); // SaddleBrown
    let c2 = Some(Color::Rgb { r: 205, g: 133, b: 63 }); // Peru (lighter)
    let c3 = Some(Color::Rgb { r: 255, g: 69, b: 0 });   // RedOrange (eyes)

    let pattern = [
        0, 0, 1, 1, 1, 1, 0, 0,
        0, 1, 1, 1, 1, 1, 1, 0,
        1, 1, 3, 1, 1, 3, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1,
        0, 1, 2, 2, 2, 2, 1, 0,
        0, 1, 2, 1, 1, 2, 1, 0,
        0, 1, 1, 0, 0, 1, 1, 0,
        0, 1, 0, 0, 0, 0, 1, 0,
    ];

    for (i, &p) in pattern.iter().enumerate() {
        pixels[i] = match p {
            1 => c1,
            2 => c2,
            3 => c3,
            _ => None,
        };
    }
    
    Sprite::new(width, height, pixels)
}

fn create_imp_sprite_frame2() -> Sprite {
    // 8x8 Imp Sprite (Frame 2 - Arms up)
    let width = 8;
    let height = 8;
    let mut pixels = vec![None; width * height];
    
    let c1 = Some(Color::Rgb { r: 139, g: 69, b: 19 }); 
    let c2 = Some(Color::Rgb { r: 205, g: 133, b: 63 }); 
    let c3 = Some(Color::Rgb { r: 255, g: 69, b: 0 });   

    let pattern = [
        0, 0, 1, 1, 1, 1, 0, 0,
        0, 1, 1, 1, 1, 1, 1, 0,
        1, 1, 3, 1, 1, 3, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 2, 2, 2, 2, 1, 1, // Arms wider
        1, 0, 2, 1, 1, 2, 0, 1,
        0, 0, 1, 0, 0, 1, 0, 0,
        0, 0, 1, 0, 0, 1, 0, 0,
    ];

    for (i, &p) in pattern.iter().enumerate() {
        pixels[i] = match p {
            1 => c1,
            2 => c2,
            3 => c3,
            _ => None,
        };
    }
    
    Sprite::new(width, height, pixels)
}

fn create_demon_sprite_frame1() -> Sprite {
    // 10x10 Demon Sprite (Frame 1)
    let width = 10;
    let height = 10;
    let mut pixels = vec![None; width * height];
    
    let c1 = Some(Color::Rgb { r: 178, g: 34, b: 34 });  
    let c2 = Some(Color::Rgb { r: 255, g: 105, b: 180 }); 
    let c3 = Some(Color::Rgb { r: 50, g: 205, b: 50 });  

    let pattern = [
        0, 0, 0, 1, 1, 1, 1, 0, 0, 0,
        0, 0, 1, 1, 2, 2, 1, 1, 0, 0,
        0, 1, 1, 1, 2, 2, 1, 1, 1, 0,
        1, 1, 3, 1, 1, 1, 1, 3, 1, 1,
        1, 1, 1, 1, 2, 2, 1, 1, 1, 1,
        1, 2, 2, 2, 2, 2, 2, 2, 2, 1,
        0, 1, 1, 1, 2, 2, 1, 1, 1, 0,
        0, 1, 1, 0, 1, 1, 0, 1, 1, 0,
        0, 1, 0, 0, 1, 1, 0, 0, 1, 0,
        0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
    ];

    for (i, &p) in pattern.iter().enumerate() {
        pixels[i] = match p {
            1 => c1,
            2 => c2,
            3 => c3,
            _ => None,
        };
    }
    
    Sprite::new(width, height, pixels)
}

fn create_demon_sprite_frame2() -> Sprite {
    // 10x10 Demon Sprite (Frame 2 - Mouth open/move)
    let width = 10;
    let height = 10;
    let mut pixels = vec![None; width * height];
    
    let c1 = Some(Color::Rgb { r: 178, g: 34, b: 34 });  
    let c2 = Some(Color::Rgb { r: 255, g: 105, b: 180 }); 
    let c3 = Some(Color::Rgb { r: 50, g: 205, b: 50 });  

    let pattern = [
        0, 0, 0, 1, 1, 1, 1, 0, 0, 0,
        0, 0, 1, 1, 2, 2, 1, 1, 0, 0,
        0, 1, 1, 1, 2, 2, 1, 1, 1, 0,
        1, 1, 3, 1, 1, 1, 1, 3, 1, 1,
        1, 1, 1, 1, 2, 2, 1, 1, 1, 1,
        1, 2, 2, 2, 0, 0, 2, 2, 2, 1, // Mouth open
        0, 1, 1, 1, 2, 2, 1, 1, 1, 0,
        0, 1, 1, 0, 1, 1, 0, 1, 1, 0,
        0, 0, 1, 0, 1, 1, 0, 1, 0, 0, // Legs move
        0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
    ];

    for (i, &p) in pattern.iter().enumerate() {
        pixels[i] = match p {
            1 => c1,
            2 => c2,
            3 => c3,
            _ => None,
        };
    }
    
    Sprite::new(width, height, pixels)
}

fn create_projectile_sprite() -> Sprite {
    // 4x4 Projectile (Fireball)
    let width = 4;
    let height = 4;
    let mut pixels = vec![None; width * height];
    
    let c1 = Some(Color::Rgb { r: 255, g: 69, b: 0 });   // RedOrange
    let c2 = Some(Color::Rgb { r: 255, g: 215, b: 0 });  // Gold

    let pattern = [
        0, 1, 1, 0,
        1, 2, 2, 1,
        1, 2, 2, 1,
        0, 1, 1, 0,
    ];

    for (i, &p) in pattern.iter().enumerate() {
        pixels[i] = match p {
            1 => c1,
            2 => c2,
            _ => None,
        };
    }
    
    Sprite::new(width, height, pixels)
}

fn create_projectile_pistol() -> Sprite {
    // 2x2 Small Projectile
    let width = 2;
    let height = 2;
    let mut pixels = vec![None; width * height];
    
    let c1 = Some(Color::Rgb { r: 255, g: 255, b: 100 }); // Yellow

    let pattern = [
        1, 1,
        1, 1,
    ];

    for (i, &p) in pattern.iter().enumerate() {
        pixels[i] = match p {
            1 => c1,
            _ => None,
        };
    }
    
    Sprite::new(width, height, pixels)
}

fn create_projectile_shotgun() -> Sprite {
    // 3x3 Projectile
    let width = 3;
    let height = 3;
    let mut pixels = vec![None; width * height];
    
    let c1 = Some(Color::Rgb { r: 255, g: 100, b: 0 }); // Orange

    let pattern = [
        0, 1, 0,
        1, 1, 1,
        0, 1, 0,
    ];

    for (i, &p) in pattern.iter().enumerate() {
        pixels[i] = match p {
            1 => c1,
            _ => None,
        };
    }
    
    Sprite::new(width, height, pixels)
}

fn create_projectile_gatling() -> Sprite {
    // 2x2 Blue Projectile
    let width = 3;
    let height = 4;
    let mut pixels = vec![None; width * height];
    
    let c1 = Some(Color::Rgb { r: 100, g: 200, b: 255 }); // Light Blue

    let pattern = [
        0, 0, 0,
        0, 0, 0,
        0, 1, 0,
        0, 0, 0,
    ];

    for (i, &p) in pattern.iter().enumerate() {
        pixels[i] = match p {
            1 => c1,
            _ => None,
        };
    }
    
    Sprite::new(width, height, pixels)
}
