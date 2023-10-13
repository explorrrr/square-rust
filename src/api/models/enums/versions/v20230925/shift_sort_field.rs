//! ShiftSortField Enum

use serde::{Deserialize, Serialize};

/// Enumerates the Shift fields to sort on.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShiftSortFieldV20230925 {
    /// The start date/time of a Shift
    StartAt,
    /// The end date/time of a Shift
    EndAt,
    /// The date/time that a Shift is created
    CreatedAt,
    /// The most recent date/time that a Shift is updated
    UpdatedAt,
}
