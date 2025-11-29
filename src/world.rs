use std::io::Result;
use std::collections::HashMap;
use crate::entity::{Entity, EntityType, Transform, SpriteType};
use crate::level::Level;

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

    pub fn try_move_entity(&mut self, entity_id: u32, new_x: f64, new_y: f64, level: &Level) -> bool {
        if self.can_move_to(new_x, new_y, level) {
            if let Some(entity) = self.entities.get_mut(&entity_id) {
                entity.transform.x = new_x;
                entity.transform.y = new_y;
                return true;
            }
        }
        false
    }

    pub fn rotate_entity(&mut self, entity_id: u32, angle_delta: f64) {
        if let Some(entity) = self.entities.get_mut(&entity_id) {
            entity.transform.angle += angle_delta;
            if entity.transform.angle >= 360.0 {
                entity.transform.angle -= 360.0;
            } else if entity.transform.angle < 0.0 {
                entity.transform.angle += 360.0;
            }
        }
    }

    pub fn move_entity_forward(&mut self, entity_id: u32, distance: f64, level: &Level) -> bool {
        if let Some(entity) = self.entities.get(&entity_id).cloned() {
            let radians = entity.transform.angle.to_radians();
            let new_x = entity.transform.x + radians.cos() * distance;
            let new_y = entity.transform.y + radians.sin() * distance;
            self.try_move_entity(entity_id, new_x, new_y, level)
        } else {
            false
        }
    }

    pub fn strafe_entity(&mut self, entity_id: u32, distance: f64, level: &Level) -> bool {
        if let Some(entity) = self.entities.get(&entity_id).cloned() {
            let radians = (entity.transform.angle + 90.0).to_radians();
            let new_x = entity.transform.x + radians.cos() * distance;
            let new_y = entity.transform.y + radians.sin() * distance;
            self.try_move_entity(entity_id, new_x, new_y, level)
        } else {
            false
        }
    }

    fn can_move_to(&self, x: f64, y: f64, level: &Level) -> bool {
        let grid_x = x as usize;
        let grid_y = y as usize;
        
        if grid_x < level.layout[0].len() && grid_y < level.layout.len() {
            level.layout[grid_y][grid_x] == 0
        } else {
            false
        }
    }

    pub fn spawn_projectile(&mut self, x: f64, y: f64, angle: f64) -> u32 {
        let projectile = Entity::new_projectile(0, x, y, angle);
        self.spawn_entity(projectile)
    }

    pub fn update(&mut self, delta_time: f64, level: &Level) {
        let mut entities_to_remove = Vec::new();
        let mut projectile_updates = Vec::new();
        
        // Collect projectile updates first
        for entity in self.entities.values() {
            if !entity.active {
                continue;
            }

            match entity.entity_type {
                EntityType::Projectile => {
                    let radians = entity.transform.angle.to_radians();
                    let new_x = entity.transform.x + radians.cos() * entity.speed * delta_time;
                    let new_y = entity.transform.y + radians.sin() * entity.speed * delta_time;
                    
                    // Check collision with walls
                    if self.can_move_to(new_x, new_y, level) {
                        projectile_updates.push((entity.id, new_x, new_y));
                    } else {
                        // Projectile hit wall - mark for removal
                        entities_to_remove.push(entity.id);
                    }
                    
                    // Remove projectiles that travel too far
                    if new_x < 0.0 || new_y < 0.0 || new_x > 24.0 || new_y > 24.0 {
                        entities_to_remove.push(entity.id);
                    }
                }
                _ => {}
            }
        }
        
        // Apply projectile position updates
        for (id, new_x, new_y) in projectile_updates {
            if let Some(entity) = self.entities.get_mut(&id) {
                entity.transform.x = new_x;
                entity.transform.y = new_y;
            }
        }
        
        // Remove dead projectiles
        for id in &entities_to_remove {
            self.entities.remove(id);
        }
        entities_to_remove.clear();

        // Update animations and states
        for entity in self.entities.values_mut() {
            if !entity.active { continue; }

            // Handle State Transitions
            match entity.state {
                crate::entity::EntityState::Hit => {
                    entity.animation_timer += delta_time;
                    if entity.animation_timer >= 0.2 { // Hit flash duration
                        entity.state = crate::entity::EntityState::Idle;
                        entity.animation_timer = 0.0;
                    }
                },
                crate::entity::EntityState::Dying => {
                    entity.animation_timer += delta_time;
                    if entity.animation_timer >= 0.5 { // Death animation duration
                        entity.state = crate::entity::EntityState::Dead;
                        entity.active = false; // Mark for removal
                    }
                },
                _ => {
                    // Normal animation for Idle/Moving
                    entity.animation_timer += delta_time;
                    let duration = crate::sprites::get_animation_duration(entity.sprite_type);
                    if entity.animation_timer >= duration {
                        entity.animation_timer -= duration;
                        entity.current_frame += 1;
                    }
                }
            }
        }

        // Collision Detection: Projectiles vs Enemies
        let mut hits = Vec::new();
        
        // Collect active projectiles and enemies
        let projectiles: Vec<(u32, f64, f64)> = self.entities.values()
            .filter(|e| e.entity_type == EntityType::Projectile && e.active)
            .map(|e| (e.id, e.transform.x, e.transform.y))
            .collect();
            
        let enemies: Vec<(u32, f64, f64)> = self.entities.values()
            .filter(|e| e.entity_type == EntityType::Enemy && e.active && e.state != crate::entity::EntityState::Dying && e.state != crate::entity::EntityState::Dead)
            .map(|e| (e.id, e.transform.x, e.transform.y))
            .collect();

        for (p_id, p_x, p_y) in projectiles {
            for (e_id, e_x, e_y) in &enemies {
                let dist_sq = (p_x - e_x).powi(2) + (p_y - e_y).powi(2);
                if dist_sq < 0.1 { // Hit radius squared
                    hits.push((p_id, *e_id));
                    break; // Projectile hits first enemy
                }
            }
        }

        // Apply hits
        for (p_id, e_id) in hits {
            // Remove projectile
            if let Some(proj) = self.entities.get_mut(&p_id) {
                proj.active = false;
            }
            entities_to_remove.push(p_id);

            // Damage enemy
            if let Some(enemy) = self.entities.get_mut(&e_id) {
                enemy.take_damage(20); // Pistol damage
            }
        }
        
        // Remove dead entities
        let mut dead_ids = Vec::new();
        for (id, entity) in &self.entities {
            if !entity.active {
                dead_ids.push(*id);
            }
        }
        for id in dead_ids {
            self.entities.remove(&id);
        }
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