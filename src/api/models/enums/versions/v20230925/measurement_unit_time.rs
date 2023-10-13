//! MeasurementUnitTime Enum

use serde::{Deserialize, Serialize};

/// Unit of time used to measure a quantity (a duration).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MeasurementUnitTimeV20230925 {
    /// The time is measured in milliseconds.
    GenericMillisecond,
    /// The time is measured in seconds.
    GenericSecond,
    /// The time is measured in minutes.
    GenericMinute,
    /// The time is measured in hours.
    GenericHour,
    /// The time is measured in days.
    GenericDay,
}
