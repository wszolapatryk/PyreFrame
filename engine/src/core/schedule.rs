use crate::ecs::system::System;

/// A sequence of systems to run in order.
/// Systems are executed deterministically in the order they were added.
pub struct Schedule {
    systems: Vec<System>,
}

impl Schedule {
    /// Creates a new, empty schedule.
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    /// Adds a system to the end of the schedule.
    pub fn add_system(&mut self, system: System) {
        self.systems.push(system);
    }

    /// Runs all systems in the schedule on the given world.
    pub fn run(&self, world: &mut crate::World) {
        for system in &self.systems {
            system(world);
        }
    }
}
