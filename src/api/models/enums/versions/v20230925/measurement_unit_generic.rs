//! MeasurementUnitGeneric Enum

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MeasurementUnitGenericV20230925 {
    /// The generic unit.
    Unit,
}
