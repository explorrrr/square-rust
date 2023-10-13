//! StandardUnitDescription

use serde::{Deserialize, Serialize};

use super::measurement_unit::MeasurementUnitV20230925;

/// Contains the name and abbreviation for standard measurement unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardUnitDescriptionV20230925 {
    /// Identifies the measurement unit being described.
    pub unit: Option<MeasurementUnitV20230925>,
    /// UI display name of the measurement unit. For example, 'Pound'.
    pub name: Option<String>,
    /// UI display abbreviation for the measurement unit. For example, 'lb'.
    pub abbreviation: Option<String>,
}
