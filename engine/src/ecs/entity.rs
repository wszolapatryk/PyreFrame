//! ECS entities.
//!
//! Entity definitions and handles.
//! Unique identifiers for game objects.
//! Supports generational indexing for safety.

/// A handle to an entity in the ECS world.
/// Uses generational indexing to prevent stale references.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Entity {
    pub id: usize,
    pub generation: usize,
}

impl Entity {
    /// Creates a new entity with the given ID and generation.
    pub fn new(id: usize, generation: usize) -> Self {
        Self { id, generation }
    }
}
