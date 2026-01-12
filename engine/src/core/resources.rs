//! Resources container.
//!
//! Holds shared resources like time and input.
//! Passed to systems or used by the engine.

use crate::input::Input;
use crate::time::Time;

/// Aggregates shared resources for the engine.
/// Includes time and input state for the current frame.
pub struct Resources {
    pub time: Time,
    pub input: Input,
}
