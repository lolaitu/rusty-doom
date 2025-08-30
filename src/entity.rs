use std::io::Result;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntityType {
    Player,
    Enemy,
    Projectile,
}

#[derive(Clone)]
pub struct Entity {
    pub id: u32,
    pub entity_type: EntityType,
    pub transform: Transform,
    pub speed: f64,
    pub health: i32,
    pub active: bool,
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
        }
    }

    pub fn new_enemy(id: u32, x: f64, y: f64) -> Self {
        Self {
            id,
            entity_type: EntityType::Enemy,
            transform: Transform::new(x, y, 0.0),
            speed: 0.1,
            health: 50,
            active: true,
        }
    }
}