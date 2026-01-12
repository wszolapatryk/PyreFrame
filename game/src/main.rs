//! PyreFrame example game.
//!
//! Game-specific initialization.
//! Registers systems and assets.
//! Defines gameplay behavior.
//! Uses the engine as a library.

use pyreframe_engine::ecs::system::movement_system;
use pyreframe_engine::{
    World,
    ecs::components::{Color, Position, Velocity},
};

fn main() {
    let mut world = World::new();

    // Spawn player entity and add components
    let player = world.spawn();

    world.insert_component(
        player,
        Position {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        },
    );
    world.insert_component(
        player,
        Velocity {
            dx: 1.0,
            dy: 0.5,
            dz: 2.0,
        },
    );
    world.insert_component(
        player,
        Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        },
    );

    // Add a simple movement system (see below)
    world.add_system(movement_system);

    // Run systems (this will update the entity's position)
    world.run_systems();

    // Example: Query the updated position
    if let Some(pos) = world.get_component::<Position>(player) {
        println!("Entity position: ({}, {}, {})", pos.x, pos.y, pos.z);
    }
}
