use pyreframe_engine::ecs::World;

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
