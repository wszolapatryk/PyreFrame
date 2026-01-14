use pyreframe_engine::ecs::World;
use pyreframe_engine::ecs::components::{Position, Velocity};
use pyreframe_engine::ecs::system::{movement_system, time_system};
use pyreframe_engine::time::{FrameDelta, Time};

#[test]
fn movement_system_moves_entities_with_position_and_velocity() {
    let mut world = World::new();
    let entity = world.spawn();
    world.insert_component(entity, Position { x: 1.0, y: 1.0, z: 0.0 }).unwrap();
    world.insert_component(entity, Velocity { dx: 1.0, dy: 0.5, dz: 0.0 }).unwrap();
    movement_system(&mut world);
    let pos = world.get_component::<Position>(entity).unwrap();
    assert_eq!(pos, &Position { x: 2.0, y: 1.5, z: 0.0 });
}

#[test]
fn initialize_velocity_system() {
    let mut world = World::new();
    let entity = world.spawn();
    world.insert_component(entity, Position { x: 1.0, y: 1.0, z: 0.0 }).unwrap();
    assert_eq!(world.has_component::<Velocity>(entity), Ok(false));
    for e in world.entities_with::<(Position,)>() {
        if world.has_component::<Velocity>(e) == Ok(false) {
            world.insert_component(e, Velocity { dx: 2.0, dy: 1.0, dz: 0.0 }).unwrap();
        }
    }
    assert!(world.has_component::<Velocity>(entity).unwrap());
    let vel = world.get_component::<Velocity>(entity).unwrap();
    assert_eq!(vel, &Velocity { dx: 2.0, dy: 1.0, dz: 0.0 });
}

#[test]
fn time_system_advances_time() {
    let mut world = World::new();
    world.insert_resource(Time::default());
    world.insert_resource(FrameDelta { dt: 0.016 });
    time_system(&mut world);
    let time = world.get_resource::<Time>().unwrap();
    assert_eq!(time.frame, 1);
    assert_eq!(time.delta, 0.016);
}

#[test]
fn time_system_increments_frame_each_call() {
    let mut world = World::new();
    world.insert_resource(Time::default());
    world.insert_resource(FrameDelta { dt: 0.016 });
    time_system(&mut world);
    world.insert_resource(FrameDelta { dt: 0.020 });
    time_system(&mut world);
    let time = world.get_resource::<Time>().unwrap();
    assert_eq!(time.frame, 2);
    assert_eq!(time.delta, 0.020);
}
