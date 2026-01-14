//! Time management.
//!
//! Delta time and frame timing.
//! Fixed and variable timestep support.
//! Time-related utilities.

/// Represents the time state for the current frame.
///
/// Tracks the delta time since the last frame and the total frame count.
/// Used as a resource in the ECS world to provide timing information to systems.
#[derive(Debug, Clone, Copy)]
pub struct Time {
    /// Delta time (in seconds) since the last frame.
    pub delta: f32,
    /// Total number of frames since start.
    pub frame: u64,
}

/// Resource representing the delta time for the current frame.
///
/// Typically set at the start of each frame and used by systems to advance simulation.
#[derive(Debug, Clone, Copy)]
pub struct FrameDelta {
    /// Delta time (in seconds) for this frame.
    pub dt: f32,
}

impl Time {
    /// Advances the time state by the given delta.
    ///
    /// Sets `delta` to `dt` and increments the frame count.
    pub fn advance(&mut self, dt: f32) {
        self.delta = dt;
        self.frame += 1;
    }
}

impl Default for Time {
    /// Returns a zero-initialized time state (delta = 0.0, frame = 0).
    fn default() -> Self {
        Self {
            delta: 0.0,
            frame: 0,
        }
    }
}

impl Default for FrameDelta {
    /// Returns a zero-initialized frame delta (dt = 0.0).
    fn default() -> Self {
        Self { dt: 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use crate::time::Time;

    #[test]
    fn test_time_advance() {
        let mut time = Time::default();
        assert_eq!(time.delta, 0.0);
        assert_eq!(time.frame, 0);

        time.advance(0.016);
        assert_eq!(time.delta, 0.016);
        assert_eq!(time.frame, 1);

        time.advance(0.033);
        assert_eq!(time.delta, 0.033);
        assert_eq!(time.frame, 2);
    }
}
