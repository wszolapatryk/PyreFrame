
/// Represents the position of the mouse cursor.
#[derive(Debug, Clone, Copy, Default)]
pub struct MousePosition {
    pub x: f32,
    pub y: f32,
}

/// Represents the state of mouse buttons.
#[derive(Debug, Clone, Copy, Default)]
pub struct MouseButtons {
    pub left: bool,
    pub right: bool,
    pub middle: bool,
}

/// Tracks the state of mouse input.
#[derive(Debug, Clone, Copy, Default)]
pub struct MouseState {
    pub position: MousePosition,
    pub buttons: MouseButtons,
    pub wheel_delta: f32,
}
