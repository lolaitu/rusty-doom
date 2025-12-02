use crate::common::world::World;
use crate::common::entity::Entity;
use std::collections::HashMap;

pub struct SnapshotInterpolator {
    pub snapshots: Vec<(f64, World)>, // timestamp, world state
}

impl SnapshotInterpolator {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
        }
    }

    pub fn add_snapshot(&mut self, timestamp: f64, world: World) {
        self.snapshots.push((timestamp, world));
        // Keep only recent snapshots
        if self.snapshots.len() > 10 {
            self.snapshots.remove(0);
        }
    }

    pub fn interpolate(&self, render_time: f64) -> Option<World> {
        // Find two snapshots surrounding render_time
        // For now, just return the latest for simplicity
        // TODO: Implement actual interpolation
        self.snapshots.last().map(|(_, w)| w.clone())
    }
}
