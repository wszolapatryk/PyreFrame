//! ECS world.
//!
//! Entity storage and component storage.
//! Ownership of all ECS data.
//! Provides controlled access for systems.
//! No game logic lives here.

use super::entity::Entity;
use super::system::System;

/// This minimal version tracks entity count and generations for generational indexing.
#[derive(Debug, Default)]
pub struct World {
    entity_count: usize,
    systems: Vec<System>,
    generations: Vec<usize>,
}

impl World {
    /// Creates a new, empty world.
    pub fn new() -> Self {
        Self {
            entity_count: 0,
            systems: Vec::new(),
            generations: Vec::new(),
        }
    }

    /// Spawns a new entity and returns its handle.
    pub fn spawn(&mut self) -> Entity {
        let id = self.entity_count;
        self.entity_count += 1;
        if id >= self.generations.len() {
            self.generations.push(0);
        }
        Entity::new(id, self.generations[id])
    }

    /// Returns the total number of entities ever spawned.
    pub fn entity_count(&self) -> usize {
        self.entity_count
    }

    /// Register a system to be run later.
    pub fn add_system(&mut self, system: System) {
        self.systems.push(system);
    }

    /// Run all registered systems in order.
    pub fn run_systems(&mut self) {
        let systems: Vec<System> = self.systems.iter().copied().collect();
        for system in systems {
            system(self);
        }
    }
}
