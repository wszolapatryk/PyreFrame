//! ECS systems.
//!
//! System definitions and signatures.
//! System scheduling hooks.
//! Explicit data dependencies.
//! Keeps systems simple and predictable.

use super::World;

/// A system is a simple function that takes a mutable reference to `World`.
pub type System = fn(&mut World);

#[cfg(test)]
fn spawn_system(world: &mut World) {
    world.spawn();
}

#[test]
fn system_can_spawn_entity() {
    let mut world = World::new();

    world.add_system(spawn_system);
    world.run_systems();

    assert_eq!(world.entity_count(), 1);
}