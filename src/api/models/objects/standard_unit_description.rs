//! StandardUnitDescription

use serde::{Deserialize, Serialize};

/// Contains the name and abbreviation for standard measurement unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardUnitDescription {
    /// Identifies the measurement unit being described.
    pub unit: Option<MeasurementUnit>,
    /// UI display name of the measurement unit. For example, 'Pound'.
    pub name: Option<String>,
    /// UI display abbreviation for the measurement unit. For example, 'lb'.
    pub abbreviation: Option<String>,
}
