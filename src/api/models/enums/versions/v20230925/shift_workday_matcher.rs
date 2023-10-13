//! ShiftWorkdayMatcher Enum

use serde::{Deserialize, Serialize};

/// Defines the logic used to apply a workday filter.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShiftWorkdayMatcherV20230925 {
    /// All shifts that start on or after the specified workday
    StartAt,
    /// All shifts that end on or before the specified workday
    EndAt,
    /// All shifts that start between the start and end workdays (inclusive)
    Intersection,
}
