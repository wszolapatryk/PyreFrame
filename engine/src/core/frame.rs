use crate::render::RenderCommand;

/// Output from a single frame of engine execution.
/// Contains all rendering commands to be processed.
pub struct FrameOutput {
    pub render_commands: Vec<RenderCommand>,
}
