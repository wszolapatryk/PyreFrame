//! Time management.
//!
//! Delta time and frame timing.
//! Fixed and variable timestep support.
//! Time-related utilities.

/// Represents time state for the current frame.
/// Includes delta time since last frame and total frame count.
pub struct Time {
    pub delta: f32,
    pub frame: u64,
}
