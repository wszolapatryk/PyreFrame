use crate::World;
use crate::core::schedule::Schedule;
use crate::input::Input;
use crate::core::frame::FrameOutput;

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
    pub fn tick(&mut self, _input: Input, _dt: f32) -> FrameOutput {
        // deterministic execution
        self.schedule.run(&mut self.world);
        FrameOutput {
            render_commands: Vec::new(),
        }
    }
}
