//! DayOfWeek Enum

use serde::{Deserialize, Serialize};

/// Indicates the specific day of the week.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DayOfWeek {
    /// Sunday
    Sun,
    /// Monday
    Mon,
    /// Tuesday
    Tue,
    /// Wednesday
    Wed,
    /// Thursday
    Thu,
    /// Friday
    Fri,
    /// Saturday
    Sat,
}
