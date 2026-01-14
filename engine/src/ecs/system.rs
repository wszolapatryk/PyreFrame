//! ECS systems.
//!
//! System definitions and signatures.
//! System scheduling hooks.
//! Explicit data dependencies.
//! Keeps systems simple and predictable.

use crate::{
    World,
    ecs::components::{Position, Velocity},
    time::{FrameDelta, Time},
};

/// A system is a function that operates on the ECS [`crate::ecs::world::World`].
///
/// Systems are registered with the world and called each frame to update game state.
/// They may mutate components, resources, or perform logic, but should be deterministic and explicit.
pub type System = fn(&mut World);

/// A basic movement system that updates [`crate::ecs::components::Position`] based on [`crate::ecs::components::Velocity`].
///
/// Iterates over all entities with both `Position` and `Velocity` components,
/// adding velocity to position for each axis. This is a simple example of a system
/// that operates on a subset of entities and mutates their state.
///
/// # Panics
///
/// Panics if a required component is missing for an entity (should not occur if ECS is consistent).
pub fn movement_system(world: &mut World) {
    let entities = world.entities_with::<(Position, Velocity)>();

    for entity in entities {
        let vel = *world.get_component::<Velocity>(entity).unwrap();
        let pos = world.get_component_mut::<Position>(entity).unwrap();

        pos.x += vel.dx;
        pos.y += vel.dy;
        pos.z += vel.dz;
    }
}

/// A system that advances the [`crate::time::Time`] resource using the current [`crate::time::FrameDelta`].
///
/// This system should be run once per frame, after the frame delta is set.
/// It updates the `Time` resource's `delta` and increments the frame count.
///
/// # Panics
///
/// Panics if either `FrameDelta` or `Time` resources are missing from the world.
pub fn time_system(world: &mut World) {
    let dt = world
        .get_resource::<FrameDelta>()
        .expect("FrameDelta must exist")
        .dt;

    let time = world.get_resource_mut::<Time>().expect("Time must exist");

    time.advance(dt);
}
