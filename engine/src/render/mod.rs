//! Rendering subsystem.
//!
//! Rendering API abstraction.
//! Render pipelines and frame submission.
//! Decoupled from ECS and game logic.

/// A 3D transformation for rendering.
/// Can be used for 2D by setting z=0 and ignoring depth.
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// An identifier for a mesh resource.
#[derive(Debug, Clone, Copy)]
pub struct MeshId(pub usize);

/// A command to render a mesh at a specific transform.
pub struct RenderCommand {
    pub transform: Transform,
    pub mesh: MeshId,
}
