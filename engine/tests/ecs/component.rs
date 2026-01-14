use pyreframe_engine::ecs::World;
use pyreframe_engine::ecs::components::{Position, Velocity};

#[test]
fn component_can_be_inserted_read_and_mutated() {
    let mut world = World::new();
    let entity = world.spawn();

    world
        .insert_component(
            entity,
            Position {
                x: 10.0,
                y: 20.0,
                z: 0.0,
            },
        )
        .unwrap();

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
    world
        .insert_component(
            e1,
            Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        )
        .unwrap();

    let e2 = world.spawn();
    world
        .insert_component(
            e2,
            Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        )
        .unwrap();
    world
        .insert_component(
            e2,
            Velocity {
                dx: 1.0,
                dy: 1.0,
                dz: 0.0,
            },
        )
        .unwrap();

    let result = world.entities_with::<(Position, Velocity)>();
    assert_eq!(result, vec![e2]);
}

#[test]
fn component_can_be_removed() {
    let mut world = World::new();
    let entity = world.spawn();

    world
        .insert_component(
            entity,
            Position {
                x: 5.0,
                y: 10.0,
                z: 15.0,
            },
        )
        .unwrap();
    assert!(world.has_component::<Position>(entity).unwrap());
    assert!(world.get_component::<Position>(entity).is_ok());

    let removed = world.remove_component::<Position>(entity).unwrap();
    let pos = removed.downcast::<Position>().ok().unwrap();
    assert_eq!(pos.x, 5.0);
    assert_eq!(pos.y, 10.0);
    assert_eq!(pos.z, 15.0);

    assert!(!world.has_component::<Position>(entity).unwrap());
    assert!(world.get_component::<Position>(entity).is_err());
}
