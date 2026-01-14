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

    let player = world.spawn();

    let _ = world.insert_component(
        player,
        Position {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        },
    );
    let _ = world.insert_component(
        player,
        Velocity {
            dx: 1.0,
            dy: 0.5,
            dz: 2.0,
        },
    );
    let _ = world.insert_component(
        player,
        Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        },
    );

    world.add_system(movement_system);

    world.run_systems();

    if let Ok(pos) = world.get_component::<Position>(player) {
        println!("Entity position: ({}, {}, {})", pos.x, pos.y, pos.z);
    }
}
