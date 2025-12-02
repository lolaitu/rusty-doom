use crate::weapon::Weapon;
use std::io::Result;

use std::time::{Duration, Instant};

pub struct Player {
    pub entity_id: u32,
    pub max_speed: f64,
    pub max_rotation_speed: f64,
    pub health: u32,
    pub weapons: Vec<Weapon>,
    pub current_weapon: usize,
    pub switch_cooldown_expiry: Instant,
    pub kills: u32,
    pub transform: crate::entity::Transform,
}

impl Player {
    pub fn new() -> Result<Self> {
        Ok(Self {
            entity_id: 0, // Will be set when spawned in world
            max_speed: 4.0,
            max_rotation_speed: 50.0,
            health: 100,
            weapons: vec![Weapon::new_pistol(), Weapon::new_shotgun(), Weapon::new_gatling(), Weapon::new_shotgun()],
            current_weapon: 0,
            switch_cooldown_expiry: Instant::now(),
            kills: 0,
            transform: crate::entity::Transform::new(3.5, 3.5, 0.0),
        })
    }

    pub fn get_current_weapon(&self) -> &Weapon {
        &self.weapons[self.current_weapon]
    }

    pub fn fire(&mut self) -> bool {
        if Instant::now() < self.switch_cooldown_expiry {
            return false;
        }
        self.weapons[self.current_weapon].fire()
    }

    pub fn reload(&mut self) {
        if Instant::now() < self.switch_cooldown_expiry {
            return;
        }
        self.weapons[self.current_weapon].reload()
    }

    pub fn animation_update(&mut self) {
        self.weapons[self.current_weapon].update();
    }

    pub fn switch_weapon(&mut self, index: usize) {
        if index < self.weapons.len() && index != self.current_weapon {
            self.current_weapon = index;
            self.switch_cooldown_expiry = Instant::now() + Duration::from_millis(500);
            // Reset state of previous weapon if needed? No, they keep state.
            // But maybe we should ensure the new weapon is Idle?
            // self.weapons[self.current_weapon].state = WeaponState::Idle; // Optional, but safer
        }
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.health = self.health.saturating_sub(amount);
    }

    pub fn is_firing(&self) -> bool {
        self.weapons[self.current_weapon].state == crate::weapon::WeaponState::Firing
    }
}
