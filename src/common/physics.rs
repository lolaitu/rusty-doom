use crate::common::world::World;
use crate::common::entity::{Entity, EntityType, EntityState, SpriteType};
use crate::common::level::Level;

pub struct Physics;

impl Physics {
    pub fn update(world: &mut World, delta_time: f64, level: &Level) -> u32 {
        let mut entities_to_remove = Vec::new();
        let mut projectile_updates = Vec::new();
        let mut kills = 0;
        
        // Collect projectile updates first
        let entity_ids: Vec<u32> = world.entities.keys().cloned().collect();
        
        for id in entity_ids {
            if let Some(entity) = world.entities.get_mut(&id) {
                if !entity.active {
                    continue;
                }

                match entity.entity_type {
                    EntityType::Projectile => {
                        let radians = entity.transform.angle.to_radians();
                        let dist_step = entity.speed * delta_time;
                        let new_x = entity.transform.x + radians.cos() * dist_step;
                        let new_y = entity.transform.y + radians.sin() * dist_step;
                        
                        entity.distance_traveled += dist_step;

                        // Check collision with walls
                        if Self::can_move_to(new_x, new_y, level) {
                            projectile_updates.push((entity.id, new_x, new_y));
                        } else {
                            // Projectile hit wall - mark for removal
                            entities_to_remove.push(entity.id);
                        }
                        
                        // Remove projectiles that travel too far
                        if entity.distance_traveled >= entity.max_distance {
                            entities_to_remove.push(entity.id);
                        }
                        
                        // Remove projectiles that travel out of bounds (safety)
                        if new_x < 0.0 || new_y < 0.0 || new_x > 24.0 || new_y > 24.0 {
                            entities_to_remove.push(entity.id);
                        }
                    }
                    _ => {}
                }
            }
        }
        
        // Apply projectile position updates
        for (id, new_x, new_y) in projectile_updates {
            if let Some(entity) = world.entities.get_mut(&id) {
                entity.transform.x = new_x;
                entity.transform.y = new_y;
            }
        }
        
        // Remove dead projectiles
        for id in &entities_to_remove {
            world.entities.remove(id);
        }
        entities_to_remove.clear();

        // Update animations and states
        for entity in world.entities.values_mut() {
            if !entity.active { continue; }

            // Handle State Transitions
            match entity.state {
                EntityState::Hit => {
                    entity.animation_timer += delta_time;
                    if entity.animation_timer >= 0.2 { // Hit flash duration
                        entity.state = EntityState::Idle;
                        entity.animation_timer = 0.0;
                    }
                },
                EntityState::Dying => {
                    entity.animation_timer += delta_time;
                    if entity.animation_timer >= 0.5 { // Death animation duration
                        entity.state = EntityState::Dead;
                        entity.active = false; // Mark for removal
                        if entity.entity_type == EntityType::Enemy {
                            kills += 1;
                        }
                    }
                },
                _ => {
                    // Normal animation for Idle/Moving
                    entity.animation_timer += delta_time;
                    let duration = crate::graphics::sprites::get_animation_duration(entity.sprite_type);
                    
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
        let projectiles: Vec<(u32, f64, f64, i32)> = world.entities.values()
            .filter(|e| e.entity_type == EntityType::Projectile && e.active)
            .map(|e| (e.id, e.transform.x, e.transform.y, e.damage))
            .collect();
            
        let enemies: Vec<(u32, f64, f64)> = world.entities.values()
            .filter(|e| e.entity_type == EntityType::Enemy && e.active && e.state != EntityState::Dying && e.state != EntityState::Dead)
            .map(|e| (e.id, e.transform.x, e.transform.y))
            .collect();

        for (p_id, p_x, p_y, p_damage) in projectiles {
            for (e_id, e_x, e_y) in &enemies {
                let dist_sq = (p_x - e_x).powi(2) + (p_y - e_y).powi(2);
                if dist_sq < 0.1 { // Hit radius squared
                    hits.push((p_id, *e_id, p_damage));
                    break; // Projectile hits first enemy
                }
            }
        }

        // Apply hits
        for (p_id, e_id, damage) in hits {
            // Remove projectile
            if let Some(proj) = world.entities.get_mut(&p_id) {
                proj.active = false;
            }
            entities_to_remove.push(p_id);

            // Damage enemy
            if let Some(enemy) = world.entities.get_mut(&e_id) {
                enemy.take_damage(damage);
            }
        }
        
        // Remove dead entities
        let mut dead_ids = Vec::new();
        for (id, entity) in &world.entities {
            if !entity.active {
                dead_ids.push(*id);
            }
        }
        for id in dead_ids {
            world.entities.remove(&id);
        }
        
        kills
    }

    pub fn move_entity_forward(world: &mut World, entity_id: u32, distance: f64, level: &Level) -> bool {
        if let Some(entity) = world.entities.get(&entity_id).cloned() {
            let radians = entity.transform.angle.to_radians();
            let new_x = entity.transform.x + radians.cos() * distance;
            let new_y = entity.transform.y + radians.sin() * distance;
            Self::try_move_entity(world, entity_id, new_x, new_y, level)
        } else {
            false
        }
    }

    pub fn strafe_entity(world: &mut World, entity_id: u32, distance: f64, level: &Level) -> bool {
        if let Some(entity) = world.entities.get(&entity_id).cloned() {
            let radians = (entity.transform.angle + 90.0).to_radians();
            let new_x = entity.transform.x + radians.cos() * distance;
            let new_y = entity.transform.y + radians.sin() * distance;
            Self::try_move_entity(world, entity_id, new_x, new_y, level)
        } else {
            false
        }
    }

    pub fn rotate_entity(world: &mut World, entity_id: u32, angle_delta: f64) {
        if let Some(entity) = world.entities.get_mut(&entity_id) {
            entity.transform.angle += angle_delta;
            if entity.transform.angle >= 360.0 {
                entity.transform.angle -= 360.0;
            } else if entity.transform.angle < 0.0 {
                entity.transform.angle += 360.0;
            }
        }
    }

    fn try_move_entity(world: &mut World, entity_id: u32, new_x: f64, new_y: f64, level: &Level) -> bool {
        if Self::can_move_to(new_x, new_y, level) {
            if let Some(entity) = world.entities.get_mut(&entity_id) {
                entity.transform.x = new_x;
                entity.transform.y = new_y;
                return true;
            }
        }
        false
    }

    fn can_move_to(x: f64, y: f64, level: &Level) -> bool {
        let grid_x = x as usize;
        let grid_y = y as usize;
        
        if grid_x < level.layout[0].len() && grid_y < level.layout.len() {
            level.layout[grid_y][grid_x] == 0
        } else {
            false
        }
    }
}
