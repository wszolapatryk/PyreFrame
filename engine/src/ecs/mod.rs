//! Entity-Component-System (ECS).
//!
//! Core ECS types and abstractions.
//! Entity creation and destruction.
//! System execution model.
//! Minimal, explicit, and data-oriented.

pub mod entity;
pub mod world;
pub mod system;

pub use world::World;