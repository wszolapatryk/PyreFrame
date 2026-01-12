//! ECS systems.
//!
//! System definitions and signatures.
//! System scheduling hooks.
//! Explicit data dependencies.
//! Keeps systems simple and predictable.

use crate::{
    World,
    ecs::components::{Position, Velocity},
};

/// A system is a simple function that takes a mutable reference to `World`.
/// Systems operate on the world to update components or perform logic.
pub type System = fn(&mut World);

/// A basic movement system that updates position based on velocity.
/// Iterates over entities with Position and Velocity, adding velocity to position.
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
