//! ECS world.
//!
//! Entity storage and component storage.
//! Ownership of all ECS data.
//! Provides controlled access for systems.
//! No game logic lives here.

use crate::ecs::errors::{InsertComponentError, ResourceError};
use crate::ecs::query::ComponentSet;

use super::entity::Entity;
use super::system::System;
use std::any::{Any, TypeId};
use std::collections::HashMap;

/// The ECS [`crate::ecs::world::World`] is the central storage for all entities, components, and resources.
///
/// - Owns all entity and component data.
/// - Provides controlled, explicit access for systems.
/// - No game logic or system logic lives here—only data management.
///
/// This minimal version tracks entity count and generations for generational indexing.
/// Now includes basic component storage for ECS functionality.
#[derive(Debug, Default)]
pub struct World {
    entity_count: usize,
    generations: Vec<usize>,
    systems: Vec<System>,
    components: HashMap<Entity, HashMap<TypeId, Box<dyn Any>>>,
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    /// Creates a new, empty [`crate::ecs::world::World`].
    pub fn new() -> Self {
        Self {
            entity_count: 0,
            systems: Vec::new(),
            generations: Vec::new(),
            components: HashMap::new(),
            resources: HashMap::new(),
        }
    }

    /// Spawns a new entity and returns its handle.
    ///
    /// Always creates a new entity with a unique id and generation (no id recycling).
    /// The returned [`crate::ecs::entity::Entity`] is guaranteed to be alive until despawned.
    pub fn spawn(&mut self) -> Entity {
        let id = self.entity_count;
        if id >= self.generations.len() {
            self.generations.push(0);
        }
        let entity = Entity::new(id, self.generations[id]);
        self.components.insert(entity, HashMap::new());
        self.entity_count += 1;
        entity
    }

    /// Despawns an entity, removing it and all its components from the world.
    ///
    /// Increments the entity's generation to invalidate old handles.
    /// Does not recycle ids; future entities will always get new ids.
    ///
    /// Returns `true` if the entity existed and was despawned, `false` otherwise.
    pub fn despawn(&mut self, entity: Entity) -> bool {
        if !self.is_alive(entity) {
            return false;
        }

        // Remove component storage (must exist for alive entity)
        self.components.remove(&entity);

        // Invalidate all existing handles
        self.generations[entity.id] += 1;

        true
    }

    /// Returns `true` if the given [`crate::ecs::entity::Entity`] is alive (not despawned and generation matches).
    pub fn is_alive(&self, entity: Entity) -> bool {
        self.generations
            .get(entity.id)
            .is_some_and(|&g| g == entity.generation)
    }

    /// Returns the total number of entities ever spawned (including despawned ones).
    pub fn total_spawned(&self) -> usize {
        self.entity_count
    }

    /// Returns the number of currently alive entities.
    pub fn alive_entity_count(&self) -> usize {
        self.components.len()
    }

    /// Registers a system to be run later via [`Self::run_systems`].
    pub fn add_system(&mut self, system: System) {
        self.systems.push(system);
    }

    /// Runs all registered systems in order.
    ///
    /// Each system is called with a mutable reference to the world.
    pub fn run_systems(&mut self) {
        let systems: Vec<System> = self.systems.iter().copied().collect();
        for system in systems {
            system(self);
        }
    }

    /// Inserts a component for an entity. Overwrites if it exists.
    ///
    /// Returns the old component if replaced, or an error if the entity is dead.
    pub fn insert_component<T: 'static>(
        &mut self,
        entity: Entity,
        component: T,
    ) -> Result<Option<Box<dyn Any>>, InsertComponentError> {
        if !self.is_alive(entity) {
            return Err(InsertComponentError::DeadEntity);
        }

        let entity_components = self
            .components
            .get_mut(&entity)
            .expect("alive entity must have component storage");

        Ok(entity_components.insert(TypeId::of::<T>(), Box::new(component)))
    }

    /// Gets a reference to a component for an entity.
    ///
    /// Returns `Ok(&T)` if the entity is alive and the component exists.
    /// Returns `Err(GetComponentError::DeadEntity)` if the entity is not alive.
    /// Returns `Err(GetComponentError::NotFound)` if the component does not exist for the entity.
    pub fn get_component<T: 'static>(
        &self,
        entity: Entity,
    ) -> Result<&T, crate::ecs::errors::GetComponentError> {
        if !self.is_alive(entity) {
            return Err(crate::ecs::errors::GetComponentError::DeadEntity);
        }
        let comp_map = self
            .components
            .get(&entity)
            .ok_or(crate::ecs::errors::GetComponentError::NotFound)?;
        let comp = comp_map
            .get(&TypeId::of::<T>())
            .ok_or(crate::ecs::errors::GetComponentError::NotFound)?;
        comp.downcast_ref::<T>()
            .ok_or(crate::ecs::errors::GetComponentError::NotFound)
    }

    /// Gets a mutable reference to a component for an entity.
    ///
    /// Returns `Ok(&mut T)` if the entity is alive and the component exists.
    /// Returns `Err(GetComponentMutError::DeadEntity)` if the entity is not alive.
    /// Returns `Err(GetComponentMutError::NotFound)` if the component does not exist for the entity.
    pub fn get_component_mut<T: 'static>(
        &mut self,
        entity: Entity,
    ) -> Result<&mut T, crate::ecs::errors::GetComponentMutError> {
        if !self.is_alive(entity) {
            return Err(crate::ecs::errors::GetComponentMutError::DeadEntity);
        }
        let comp_map = self
            .components
            .get_mut(&entity)
            .ok_or(crate::ecs::errors::GetComponentMutError::NotFound)?;
        let comp = comp_map
            .get_mut(&TypeId::of::<T>())
            .ok_or(crate::ecs::errors::GetComponentMutError::NotFound)?;
        comp.downcast_mut::<T>()
            .ok_or(crate::ecs::errors::GetComponentMutError::NotFound)
    }

    /// Removes a component from an entity.
    ///
    /// Returns `Ok(Box<dyn Any>)` if the entity is alive and the component existed (removed and returned).
    /// Returns `Err(RemoveComponentError::DeadEntity)` if the entity is not alive.
    /// Returns `Err(RemoveComponentError::NotFound)` if the component does not exist for the entity.
    pub fn remove_component<T: 'static>(
        &mut self,
        entity: Entity,
    ) -> Result<Box<dyn Any>, crate::ecs::errors::RemoveComponentError> {
        if !self.is_alive(entity) {
            return Err(crate::ecs::errors::RemoveComponentError::DeadEntity);
        }
        let comp_map = self
            .components
            .get_mut(&entity)
            .ok_or(crate::ecs::errors::RemoveComponentError::NotFound)?;
        comp_map
            .remove(&TypeId::of::<T>())
            .ok_or(crate::ecs::errors::RemoveComponentError::NotFound)
    }

    /// Returns all entities that have the specified set of components.
    ///
    /// Useful for systems that need to iterate over entities with certain components.
    /// The component set is specified as a tuple, e.g. `(Position, Velocity)`.
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
    ///
    /// Returns `Ok(true)` if the entity is alive and has the component.
    /// Returns `Ok(false)` if the entity is alive but does not have the component.
    /// Returns `Err(HasComponentError::DeadEntity)` if the entity is not alive.
    pub fn has_component<T: 'static>(
        &self,
        entity: Entity,
    ) -> Result<bool, crate::ecs::errors::HasComponentError> {
        if !self.is_alive(entity) {
            return Err(crate::ecs::errors::HasComponentError::DeadEntity);
        }
        Ok(self
            .components
            .get(&entity)
            .is_some_and(|c| c.contains_key(&TypeId::of::<T>())))
    }

    /// Inserts a resource of type `T` into the world, replacing any existing resource of the same type.
    ///
    /// Returns the old resource if it existed.
    pub fn insert_resource<T: 'static>(&mut self, resource: T) -> Option<T> {
        self.resources
            .insert(TypeId::of::<T>(), Box::new(resource))
            .map(|old| {
                *old.downcast::<T>().unwrap_or_else(|_| {
                    panic!(
                        "internal error: stored resource for {} had wrong type",
                        std::any::type_name::<T>()
                    )
                })
            })
    }

    /// Gets an immutable reference to a resource of type `T`.
    ///
    /// Returns `Ok(&T)` if the resource exists, or `Err(ResourceError::Missing)` if not found.
    pub fn get_resource<T: 'static>(&self) -> Result<&T, ResourceError> {
        self.resources
            .get(&TypeId::of::<T>())
            .and_then(|res| res.downcast_ref::<T>())
            .ok_or(ResourceError::Missing)
    }

    /// Gets a mutable reference to a resource of type `T`.
    ///
    /// Returns `Ok(&mut T)` if the resource exists, or `Err(ResourceError::Missing)` if not found.
    pub fn get_resource_mut<T: 'static>(&mut self) -> Result<&mut T, ResourceError> {
        self.resources
            .get_mut(&TypeId::of::<T>())
            .and_then(|res| res.downcast_mut::<T>())
            .ok_or(ResourceError::Missing)
    }
}
