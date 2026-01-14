use pyreframe_engine::ecs::World;
use pyreframe_engine::time::{FrameDelta, Time};

#[test]
fn resource_can_be_inserted_and_read() {
    let mut world = World::new();
    world.insert_resource(FrameDelta { dt: 0.016 });
    let delta = world.get_resource::<FrameDelta>().unwrap();
    assert_eq!(delta.dt, 0.016);
}

#[test]
fn inserting_resource_replaces_previous_value() {
    let mut world = World::new();
    world.insert_resource(FrameDelta { dt: 0.016 });
    let old = world.insert_resource(FrameDelta { dt: 0.033 });
    assert_eq!(old.unwrap().dt, 0.016);
    let delta = world.get_resource::<FrameDelta>().unwrap();
    assert_eq!(delta.dt, 0.033);
}

#[test]
fn resource_can_be_mutably_accessed_and_modified() {
    let mut world = World::new();
    world.insert_resource(Time::default());
    {
        let time = world.get_resource_mut::<Time>().unwrap();
        time.frame = 42;
        time.delta = 0.5;
    }
    let time = world.get_resource::<Time>().unwrap();
    assert_eq!(time.frame, 42);
    assert_eq!(time.delta, 0.5);
}

#[test]
fn getting_missing_resource_returns_error() {
    let world = World::new();
    let result = world.get_resource::<Time>();
    assert!(result.is_err());
}
