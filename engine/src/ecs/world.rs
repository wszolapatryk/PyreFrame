//! ECS world.
//!
//! Entity storage and component storage.
//! Ownership of all ECS data.
//! Provides controlled access for systems.
//! No game logic lives here.

use crate::ecs::query::ComponentSet;

use super::entity::Entity;
use super::system::System;
use std::any::{Any, TypeId};
use std::collections::HashMap;

/// This minimal version tracks entity count and generations for generational indexing.
/// Now includes basic component storage for ECS functionality.
#[derive(Debug, Default)]
pub struct World {
    entity_count: usize,
    systems: Vec<System>,
    generations: Vec<usize>,
    components: HashMap<Entity, HashMap<TypeId, Box<dyn Any>>>,
}

impl World {
    /// Creates a new, empty world.
    pub fn new() -> Self {
        Self {
            entity_count: 0,
            systems: Vec::new(),
            generations: Vec::new(),
            components: HashMap::new(),
        }
    }

    /// Spawns a new entity and returns its handle.
    pub fn spawn(&mut self) -> Entity {
        let id = self.entity_count;
        self.entity_count += 1;
        if id >= self.generations.len() {
            self.generations.push(0);
        }
        let entity = Entity::new(id, self.generations[id]);
        self.components.insert(entity, HashMap::new());
        entity
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

    /// Inserts a component for an entity. Overwrites if it exists.
    /// Returns the old component if replaced.
    pub fn insert_component<T: 'static>(
        &mut self,
        entity: Entity,
        component: T,
    ) -> Option<Box<dyn Any>> {
        if let Some(entity_components) = self.components.get_mut(&entity) {
            entity_components.insert(TypeId::of::<T>(), Box::new(component))
        } else {
            None // Entity doesn't exist (could add validation later).
        }
    }

    /// Gets a reference to a component for an entity.
    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.components
            .get(&entity)?
            .get(&TypeId::of::<T>())?
            .downcast_ref::<T>()
    }

    /// Gets a mutable reference to a component for an entity.
    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.components
            .get_mut(&entity)?
            .get_mut(&TypeId::of::<T>())?
            .downcast_mut::<T>()
    }

    /// Removes a component from an entity.
    /// Returns the component if it existed.
    pub fn remove_component<T: 'static>(&mut self, entity: Entity) -> Option<Box<dyn Any>> {
        self.components.get_mut(&entity)?.remove(&TypeId::of::<T>())
    }

    /// Returns all entities that have the specified set of components.
    /// Useful for systems that need to iterate over entities with certain components.
    pub fn entities_with<C: ComponentSet>(&self) -> Vec<Entity> {
        let required = C::type_ids();

        self.components
            .iter()
            .filter_map(|(&entity, entity_components)| {
                let has_all = required
                    .iter()
                    .all(|type_id| entity_components.contains_key(type_id));

                if has_all { Some(entity) } else { None }
            })
            .collect()
    }

    /// Checks if an entity has a specific component.
    /// Returns true if the component exists for the entity.
    pub fn has_component<T: 'static>(&self, entity: Entity) -> bool {
        self.components
            .get(&entity)
            .is_some_and(|c| c.contains_key(&TypeId::of::<T>()))
    }
}
