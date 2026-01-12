//! Entity-Component-System (ECS).
//!
//! Core ECS types and abstractions.
//! Entity creation and destruction.
//! System execution model.
//! Minimal, explicit, and data-oriented.

pub mod components;
pub mod entity;
pub mod query;
pub mod system;
pub mod world;

pub use world::World;
