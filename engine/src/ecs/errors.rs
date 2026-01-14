/// Error returned when attempting to insert a component for a dead entity.
#[derive(Debug, PartialEq, Eq)]
pub enum InsertComponentError {
    /// The entity is not alive.
    DeadEntity,
}

/// Error returned when attempting to get a component for an entity.
#[derive(Debug, PartialEq, Eq)]
pub enum GetComponentError {
    /// The entity is not alive.
    DeadEntity,
    /// The component was not found for the entity.
    NotFound,
}

/// Error returned when attempting to get a mutable reference to a component for an entity.
#[derive(Debug, PartialEq, Eq)]
pub enum GetComponentMutError {
    /// The entity is not alive.
    DeadEntity,
    /// The component was not found for the entity.
    NotFound,
}

/// Error returned when attempting to remove a component from an entity.
#[derive(Debug, PartialEq, Eq)]
pub enum RemoveComponentError {
    /// The entity is not alive.
    DeadEntity,
    /// The component was not found for the entity.
    NotFound,
}

/// Error returned when checking if an entity has a component.
#[derive(Debug, PartialEq, Eq)]
pub enum HasComponentError {
    /// The entity is not alive.
    DeadEntity,
}

/// Error returned when attempting to access a resource.
#[derive(Debug, PartialEq, Eq)]
pub enum ResourceError {
    /// The requested resource was not found in the world.
    Missing,
}
