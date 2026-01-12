use pyreframe_engine::ecs::World;
use pyreframe_engine::ecs::components::{Position, Velocity};

use pyreframe_engine::ecs::system::movement_system;

#[test]
fn world_starts_empty() {
    let world = World::new();
    assert_eq!(world.entity_count(), 0);
}

#[test]
fn spawning_entity_increases_count() {
    let mut world = World::new();
    world.spawn();
    assert_eq!(world.entity_count(), 1);
}

#[test]
fn component_can_be_inserted_read_and_mutated() {
    let mut world = World::new();
    let entity = world.spawn();

    world.insert_component(
        entity,
        Position {
            x: 10.0,
            y: 20.0,
            z: 0.0,
        },
    );

    {
        let pos = world.get_component::<Position>(entity).unwrap();
        assert_eq!(
            pos,
            &Position {
                x: 10.0,
                y: 20.0,
                z: 0.0
            }
        );
    }

    {
        let pos = world.get_component_mut::<Position>(entity).unwrap();
        pos.x = 15.0;
    }

    let pos = world.get_component::<Position>(entity).unwrap();
    assert_eq!(
        pos,
        &Position {
            x: 15.0,
            y: 20.0,
            z: 0.0
        }
    );
}

#[test]
fn entities_with_filters_by_component_presence() {
    let mut world = World::new();

    let e1 = world.spawn();
    world.insert_component(
        e1,
        Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    );

    let e2 = world.spawn();
    world.insert_component(
        e2,
        Position {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    );
    world.insert_component(
        e2,
        Velocity {
            dx: 1.0,
            dy: 1.0,
            dz: 0.0,
        },
    );

    let result = world.entities_with::<(Position, Velocity)>();

    assert_eq!(result, vec![e2]);
}

#[test]
fn movement_system_moves_entities_with_position_and_velocity() {
    let mut world = World::new();

    let entity = world.spawn();
    world.insert_component(
        entity,
        Position {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        },
    );
    world.insert_component(
        entity,
        Velocity {
            dx: 1.0,
            dy: 0.5,
            dz: 0.0,
        },
    );

    movement_system(&mut world);

    let pos = world.get_component::<Position>(entity).unwrap();
    assert_eq!(
        pos,
        &Position {
            x: 2.0,
            y: 1.5,
            z: 0.0
        }
    );
}

#[test]
fn initialize_velocity_system() {
    let mut world = World::new();
    let entity = world.spawn();

    world.insert_component(
        entity,
        Position {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        },
    );

    assert!(!world.has_component::<Velocity>(entity));

    for e in world.entities_with::<(Position,)>() {
        if !world.has_component::<Velocity>(e) {
            world.insert_component(
                e,
                Velocity {
                    dx: 2.0,
                    dy: 1.0,
                    dz: 0.0,
                },
            );
        }
    }

    assert!(world.has_component::<Velocity>(entity));

    let vel = world.get_component::<Velocity>(entity).unwrap();
    assert_eq!(
        vel,
        &Velocity {
            dx: 2.0,
            dy: 1.0,
            dz: 0.0
        }
    );
}

#[test]
fn component_can_be_removed() {
    let mut world = World::new();
    let entity = world.spawn();

    world.insert_component(
        entity,
        Position {
            x: 5.0,
            y: 10.0,
            z: 15.0,
        },
    );

    assert!(world.has_component::<Position>(entity));
    assert!(world.get_component::<Position>(entity).is_some());

    let removed = world.remove_component::<Position>(entity).unwrap();
    let pos = removed.downcast::<Position>().ok().unwrap();

    assert_eq!(pos.x, 5.0);
    assert_eq!(pos.y, 10.0);
    assert_eq!(pos.z, 15.0);

    assert!(!world.has_component::<Position>(entity));
    assert!(world.get_component::<Position>(entity).is_none());
}
