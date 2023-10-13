//! MeasurementUnit

use serde::{Deserialize, Serialize};

/// Represents a unit of measurement to use with a quantity, such as ounces or inches.
///
/// Exactly one of the following fields are required: custom_unit, area_unit, length_unit, volume_unit, and weight_unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementUnit {
    /// The information needed to define a custom unit, provided by the seller.
    pub custom_unit: Option<MeasurementUnitCustom>,
    /// Unit of area used to measure a quantity.
    pub area_unit: Option<MeasurementUnitArea>,
    /// Unit of length used to measure a quantity.
    pub length_unit: Option<MeasurementUnitLength>,
    /// Unit of volume used to measure a quantity.
    pub volume_unit: Option<MeasurementUnitVolume>,
    /// Unit of weight used to measure a quantity.
    pub weight_unit: Option<MeasurementUnitWeight>,
    /// Unit of generic measure used to measure a quantity.
    pub generic_unit: Option<MeasurementUnitGeneric>,
    /// Unit of time used to measure a quantity (a duration).
    pub time_unit: Option<MeasurementUnitTime>,
    /// Represents the type of the measurement unit.
    pub r#type: Option<MeasurementUnitUnitType>,
}

// TODO: 一つしか選べないように実装する
