//! ShiftStatus Enum

use serde::{Deserialize, Serialize};

/// Enumerates the possible status of a Shift.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShiftStatus {
    /// Employee started a work shift and the shift is not complete
    Open,
    /// Employee started and ended a work shift.
    Closed,
}
