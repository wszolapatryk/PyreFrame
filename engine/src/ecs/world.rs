//! ECS world.
//!
//! Entity storage and component storage.
//! Ownership of all ECS data.
//! Provides controlled access for systems.
//! No game logic lives here.

use super::system::System;

/// This minimal version only tracks entity count for test init.
#[derive(Debug, Default)]
pub struct World {
    entity_count: usize,
    systems: Vec<System>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_count: 0,
            systems: Vec::new(),
        }
    }

    pub fn spawn(&mut self) -> Entity {
        self.entity_count += 1;
        Entity()
    }

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Entity();
