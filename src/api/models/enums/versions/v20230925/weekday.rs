//! Weekday Enum

use serde::{Deserialize, Serialize};

/// The days of the week.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WeekdayV20230925 {
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
    /// Sunday
    Sun,
}
