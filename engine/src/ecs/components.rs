/// Position component: Represents an entity's 3D location.
/// Used for spatial queries, rendering, and collision.
/// For 2D compatibility, set z to 0.0.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Velocity component: Represents an entity's movement speed and direction in 3D.
/// Used for physics/movement systems to update position over time.
/// For 2D compatibility, set dz to 0.0.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub dx: f32, // Change in x per frame/time unit
    pub dy: f32, // Change in y per frame/time unit
    pub dz: f32, // Change in z per frame/time unit
}

/// Color component: Represents an entity's visual color for rendering.
/// Used by render systems to draw entities (e.g., sprites or shapes).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8, // Red (0-255)
    pub g: u8, // Green (0-255)
    pub b: u8, // Blue (0-255)
    pub a: u8, // Alpha (0-255, for transparency)
}
