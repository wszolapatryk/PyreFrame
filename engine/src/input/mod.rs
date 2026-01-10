//! Input handling.
//!
//! Input state abstraction.
//! Keyboard, mouse, and controller input.
//! Platform-agnostic interface.
//! No game-specific bindings.

mod key;
mod keyboard;
mod mouse;

pub use key::Key;
pub use keyboard::KeyboardState;
pub use mouse::{MouseState, MousePosition, MouseButtons};

/// Aggregates all input state for a frame.
#[derive(Debug, Clone)]
pub struct Input {
    pub keyboard: KeyboardState,
    pub mouse: MouseState,
}
