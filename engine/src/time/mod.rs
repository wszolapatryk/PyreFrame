//! Time management.
//!
//! Delta time and frame timing.
//! Fixed and variable timestep support.
//! Time-related utilities.

pub struct Time {
    pub delta: f32,
    pub frame: u64,
}
