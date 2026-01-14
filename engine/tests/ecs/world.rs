use pyreframe_engine::ecs::World;

#[test]
fn world_starts_empty() {
    let world = World::new();
    assert_eq!(world.alive_entity_count(), 0);
}

#[test]
fn spawning_entity_increases_count() {
    let mut world = World::new();
    world.spawn();
    assert_eq!(world.alive_entity_count(), 1);
}

#[test]
fn total_spawned_tracks_all_entities() {
    let mut world = World::new();
    assert_eq!(world.total_spawned(), 0);

    let e1 = world.spawn();
    let _e2 = world.spawn();
    assert_eq!(world.total_spawned(), 2);

    world.despawn(e1);
    assert_eq!(world.total_spawned(), 2);

    let _e3 = world.spawn();
    assert_eq!(world.total_spawned(), 3);
}

#[test]
fn despawning_entity_removes_it_and_components() {
    use pyreframe_engine::ecs::components::{Position, Velocity};
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
    world
        .insert_component(
            entity,
            Velocity {
                dx: 1.0,
                dy: 2.0,
                dz: 0.0,
            },
        )
        .unwrap();

    assert!(world.get_component::<Position>(entity).is_ok());
    assert!(world.get_component::<Velocity>(entity).is_ok());

    let despawned = world.despawn(entity);
    assert!(despawned);

    assert!(world.get_component::<Position>(entity).is_err());
    assert!(world.get_component::<Velocity>(entity).is_err());
}

#[test]
fn despawning_nonexistent_entity_returns_false() {
    let mut world = World::new();
    let entity = world.spawn();
    world.despawn(entity);
    let result = world.despawn(entity);
    assert!(!result);
}

#[test]
fn despawn_invalidates_entity() {
    let mut world = World::new();
    let e = world.spawn();
    assert!(world.is_alive(e));
    world.despawn(e);
    assert!(!world.is_alive(e));
}
