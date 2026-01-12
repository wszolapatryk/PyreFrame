//! PyreFrame engine public API.
//!
//! Re-exports core engine types and modules.
//! Defines what is considered stable and user-facing.
//! Keeps internal implementation details private.

pub mod core;
pub mod ecs;
pub mod input;
pub mod render;
pub mod time;

pub use core::engine::Engine;
pub use ecs::World;
pub use ecs::entity::Entity;
