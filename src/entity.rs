/* Class that manages all the world elements : players, enemies, 
projectiles*/

use std::io::Result;

// structure that stores coordinates of the entity
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
}

impl Transform {
    pub fn new(x: f64, y: f64, angle: f64) -> Self {
        Self { x, y, angle }
    }
}

// enum of the entities types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntityType {
    Player,
    Enemy,
    Projectile,
}

// structure that stores entities stats
#[derive(Clone)]
pub struct Entity {
    pub id: u32,
    pub entity_type: EntityType,
    pub transform: Transform,
    pub speed: f64,
    pub health: i32,
    pub active: bool,
    pub sprite_type: SpriteType,
    pub animation_timer: f64,
    pub current_frame: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpriteType {
    None,
    EnemyImp,
    EnemyDemon,
}

impl Entity {
    pub fn new_player(id: u32, x: f64, y: f64) -> Self {
        Self {
            id,
            entity_type: EntityType::Player,
            transform: Transform::new(x, y, 0.0),
            speed: 0.2,
            health: 100,
            active: true,
            sprite_type: SpriteType::None,
            animation_timer: 0.0,
            current_frame: 0,
        }
    }

    pub fn new_enemy(id: u32, x: f64, y: f64, sprite_type: SpriteType) -> Self {
        Self {
            id,
            entity_type: EntityType::Enemy,
            transform: Transform::new(x, y, 0.0),
            speed: 0.1,
            health: 50,
            active: true,
            sprite_type,
            animation_timer: 0.0,
            current_frame: 0,
        }
    }

    pub fn new_projectile(id: u32, x: f64, y: f64, angle: f64) -> Self {
        Self {
            id,
            entity_type: EntityType::Projectile,
            transform: Transform::new(x, y, angle),
            speed: 10.0, // Fast projectile speed
            health: 1,  // Projectile dies on impact
            active: true,
            sprite_type: SpriteType::None,
            animation_timer: 0.0,
            current_frame: 0,
        }
    }
}