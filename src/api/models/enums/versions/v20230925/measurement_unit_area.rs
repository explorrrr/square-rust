//! MeasurementUnitArea Enum

use serde::{Deserialize, Serialize};

/// Unit of area used to measure a quantity.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MeasurementUnitAreaV20230925 {
    /// The area is measured in acres.
    ImperialAcre,
    /// The area is measured in square inches.
    ImperialSquareInch,
    /// The area is measured in square feet.
    ImperialSquareFoot,
    /// The area is measured in square yards.
    ImperialSquareYard,
    /// The area is measured in square miles.
    ImperialSquareMile,
    /// The area is measured in square centimeters.
    MetricSquareCentimeter,
    /// The area is measured in square meters.
    MetricSquareMeter,
    /// The area is measured in square kilometers.
    MetricSquareKilometer,
}
