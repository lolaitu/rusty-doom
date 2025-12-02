use std::io::Result;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::entity::{Entity, EntityType, Transform, SpriteType};
use crate::level::Level;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct World {
    pub entities: HashMap<u32, Entity>,
    next_entity_id: u32,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            next_entity_id: 1,
        }
    }

    pub fn spawn_entity(&mut self, mut entity: Entity) -> u32 {
        let id = self.next_entity_id;
        entity.id = id;
        self.entities.insert(id, entity);
        self.next_entity_id += 1;
        id
    }

    pub fn get_entity_mut(&mut self, id: u32) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn get_entity(&self, id: u32) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_player(&self) -> Option<&Entity> {
        self.entities.values().find(|e| e.entity_type == EntityType::Player)
    }

    pub fn get_player_mut(&mut self) -> Option<&mut Entity> {
        self.entities.values_mut().find(|e| e.entity_type == EntityType::Player)
    }

    // spawn a projectile at a given position
    pub fn spawn_projectile(&mut self, x: f64, y: f64, angle: f64, damage: i32, max_distance: f64, sprite_type: SpriteType) -> u32 {
        let projectile = Entity::new_projectile(0, x, y, angle, damage, max_distance, sprite_type);
        self.spawn_entity(projectile)
    }

    pub fn get_projectiles(&self) -> Vec<&Entity> {
        self.entities.values()
            .filter(|e| e.entity_type == EntityType::Projectile && e.active)
            .collect()
    }

    pub fn get_enemies(&self) -> Vec<&Entity> {
        self.entities.values()
            .filter(|e| e.entity_type == EntityType::Enemy && e.active)
            .collect()
    }

    pub fn spawn_enemy(&mut self, x: f64, y: f64, sprite_type: SpriteType) -> u32 {
        let enemy = Entity::new_enemy(0, x, y, sprite_type);
        self.spawn_entity(enemy)
    }

    pub fn respawn_enemies(&mut self) {
        // Remove only enemies and projectiles, keep the player
        let ids_to_remove: Vec<u32> = self.entities.iter()
            .filter(|(_, e)| e.entity_type == EntityType::Enemy || e.entity_type == EntityType::Projectile)
            .map(|(id, _)| *id)
            .collect();

        for id in ids_to_remove {
            self.entities.remove(&id);
        }

        // Spawn default enemies
        self.spawn_enemy(10.0, 15.0, SpriteType::EnemyImp);
        self.spawn_enemy(8.0, 8.0, SpriteType::EnemyDemon);
        self.spawn_enemy(18.0, 12.0, SpriteType::EnemyImp);
    }

    pub fn reset(&mut self) {
        self.entities.clear();
        self.respawn_enemies();
    }
}