//! Keyboard input.
//!
//! Key state tracking.
//! Key press / release handling.
//! Maps platform events to engine input state.

use super::Key;

/// Tracks the state of keyboard input.
#[derive(Debug, Default, Clone)]
pub struct KeyboardState {
    pressed: Vec<Key>,
}

impl KeyboardState {
    /// Creates a new keyboard state with the given pressed keys.
    pub fn new(pressed: Vec<Key>) -> Self {
        Self { pressed }
    }

    /// Checks if a key is currently pressed.
    pub fn is_pressed(&self, key: Key) -> bool {
        self.pressed.contains(&key)
    }

    /// Returns a slice of all currently pressed keys.
    pub fn pressed_keys(&self) -> &[Key] {
        &self.pressed
    }
}
