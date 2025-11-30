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

pub const PLAYER_SPEED: f64 = 0.2;
pub const ENEMY_SPEED: f64 = 0.1;
pub const PROJECTILE_SPEED: f64 = 10.0;
pub const PLAYER_HEALTH: i32 = 100;
pub const ENEMY_HEALTH: i32 = 50;
pub const PROJECTILE_HEALTH: i32 = 1;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntityState {
    Idle,
    Hit,
    Dying,
    Dead,
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
    pub state: EntityState,
    pub max_distance: f64,
    pub distance_traveled: f64,
    pub damage: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpriteType {
    None,
    EnemyImp,
    EnemyDemon,
    ProjectilePistol,
    ProjectileShotgun,
    ProjectileGatling,
}

impl Entity {
    pub fn new_player(id: u32, x: f64, y: f64) -> Self {
        Self {
            id,
            entity_type: EntityType::Player,
            transform: Transform::new(x, y, 0.0),
            speed: PLAYER_SPEED,
            health: PLAYER_HEALTH,
            active: true,
            sprite_type: SpriteType::None,
            animation_timer: 0.0,
            current_frame: 0,
            state: EntityState::Idle,
            max_distance: 0.0,
            distance_traveled: 0.0,
            damage: 0,
        }
    }

    pub fn new_enemy(id: u32, x: f64, y: f64, sprite_type: SpriteType) -> Self {
        Self {
            id,
            entity_type: EntityType::Enemy,
            transform: Transform::new(x, y, 0.0),
            speed: ENEMY_SPEED,
            health: ENEMY_HEALTH,
            active: true,
            sprite_type,
            animation_timer: 0.0,
            current_frame: 0,
            state: EntityState::Idle,
            max_distance: 0.0,
            distance_traveled: 0.0,
            damage: 0,
        }
    }

    pub fn new_projectile(id: u32, x: f64, y: f64, angle: f64, damage: i32, max_distance: f64, sprite_type: SpriteType) -> Self {
        Self {
            id,
            entity_type: EntityType::Projectile,
            transform: Transform::new(x, y, angle),
            speed: PROJECTILE_SPEED,
            health: PROJECTILE_HEALTH,
            active: true,
            sprite_type,
            animation_timer: 0.0,
            current_frame: 0,
            state: EntityState::Idle,
            max_distance,
            distance_traveled: 0.0,
            damage,
        }
    }

    pub fn take_damage(&mut self, amount: i32) {
        if self.state == EntityState::Dying || self.state == EntityState::Dead {
            return;
        }

        self.health -= amount;
        if self.health <= 0 {
            self.state = EntityState::Dying;
            self.current_frame = 0;
            self.animation_timer = 0.0;
        } else {
            self.state = EntityState::Hit;
            self.current_frame = 0; // Reset frame to show hit effect immediately
            self.animation_timer = 0.0;
        }
    }
}