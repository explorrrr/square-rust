//! MeasurementUnitUnitType Enum

use serde::{Deserialize, Serialize};

/// Describes the type of this unit and indicates which field contains the unit information.
/// This is an ‘open’ enum.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MeasurementUnitUnitTypeV20230925 {
    /// The unit details are contained in the custom_unit field.
    TypeCustom,
    /// The unit details are contained in the area_unit field.
    TypeArea,
    /// The unit details are contained in the length_unit field.
    TypeLength,
    /// The unit details are contained in the volume_unit field.
    TypeVolume,
    /// The unit details are contained in the weight_unit field.
    TypeWeight,
    /// The unit details are contained in the generic_unit field.
    TypeGeneric,
}
