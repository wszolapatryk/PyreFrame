//! PyreFrame engine public API.
//!
//! Re-exports core engine types and modules.
//! Defines what is considered stable and user-facing.
//! Keeps internal implementation details private.

pub mod ecs;
pub use ecs::World;
