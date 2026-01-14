use crate::World;
use crate::core::frame::FrameOutput;
use crate::core::schedule::Schedule;
use crate::input::Input;
use crate::time::FrameDelta;

/// The main game engine.
/// Orchestrates the ECS world, systems, and rendering.
pub struct Engine {
    world: World,
    schedule: Schedule,
}

impl Engine {
    /// Creates a new engine with an empty world and schedule.
    pub fn new() -> Self {
        Self {
            world: World::new(),
            schedule: Schedule::new(),
        }
    }

    /// Runs all systems in the schedule once.
    pub fn run(&mut self) {
        self.schedule.run(&mut self.world);
    }

    /// Processes a single frame: updates systems and returns render output.
    pub fn tick(&mut self, _input: Input, dt: f32) -> FrameOutput {
        // Publish frame delta as a resource
        match self.world.insert_resource(FrameDelta { dt }) {
            None => {
                // First frame: FrameDelta resource inserted into the world.
            }
            Some(_previous_delta) => {
                // Subsequent frames: existing FrameDelta resource updated.
            }
        }

        // Run all systems (including TimeSystem)
        self.schedule.run(&mut self.world);

        FrameOutput {
            render_commands: Vec::new(),
        }
    }
}

impl Default for Engine {
    /// Creates a default engine (same as [`crate::core::engine::Engine::new`]).
    fn default() -> Self {
        Self::new()
    }
}
