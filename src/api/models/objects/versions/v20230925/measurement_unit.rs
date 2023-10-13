//! MeasurementUnit

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::{measurement_unit_area::MeasurementUnitAreaV20230925, measurement_unit_length::MeasurementUnitLengthV20230925, measurement_unit_volume::MeasurementUnitVolumeV20230925, measurement_unit_weight::MeasurementUnitWeightV20230925, measurement_unit_generic::MeasurementUnitGenericV20230925, measurement_unit_time::MeasurementUnitTimeV20230925, measurement_unit_unit_type::MeasurementUnitUnitTypeV20230925};

use super::measurement_unit_custom::MeasurementUnitCustomV20230925;

/// Represents a unit of measurement to use with a quantity, such as ounces or inches.
///
/// Exactly one of the following fields are required: custom_unit, area_unit, length_unit, volume_unit, and weight_unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementUnitV20230925 {
    /// The information needed to define a custom unit, provided by the seller.
    pub custom_unit: Option<MeasurementUnitCustomV20230925>,
    /// Unit of area used to measure a quantity.
    pub area_unit: Option<MeasurementUnitAreaV20230925>,
    /// Unit of length used to measure a quantity.
    pub length_unit: Option<MeasurementUnitLengthV20230925>,
    /// Unit of volume used to measure a quantity.
    pub volume_unit: Option<MeasurementUnitVolumeV20230925>,
    /// Unit of weight used to measure a quantity.
    pub weight_unit: Option<MeasurementUnitWeightV20230925>,
    /// Unit of generic measure used to measure a quantity.
    pub generic_unit: Option<MeasurementUnitGenericV20230925>,
    /// Unit of time used to measure a quantity (a duration).
    pub time_unit: Option<MeasurementUnitTimeV20230925>,
    /// Represents the type of the measurement unit.
    pub r#type: Option<MeasurementUnitUnitTypeV20230925>,
}

// TODO: 一つしか選べないように実装する
