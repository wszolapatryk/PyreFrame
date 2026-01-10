//! PyreFrame engine public API.
//!
//! Re-exports core engine types and modules.
//! Defines what is considered stable and user-facing.
//! Keeps internal implementation details private.

pub mod ecs;
pub mod core;
pub mod render;
pub mod input;
pub mod time;

pub use ecs::entity::Entity;
pub use ecs::World;
pub use core::engine::Engine;