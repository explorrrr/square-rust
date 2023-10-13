//! ShiftFilterStatus Enum

use serde::{Deserialize, Serialize};

/// Specifies the status of Shift records to be returned.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShiftFilterStatusV20230925 {
    /// Shifts that have been started and not ended.
    Open,
    /// Shifts that have been started and ended.
    Closed,
}
