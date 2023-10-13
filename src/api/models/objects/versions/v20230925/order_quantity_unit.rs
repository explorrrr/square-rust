//! OrderQuantityUnit

use serde::{Deserialize, Serialize};

use super::measurement_unit::MeasurementUnitV20230925;

/// Contains the measurement unit for a quantity and a precision that specifies the number of digits after the decimal point for decimal quantities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderQuantityUnitV20230925 {
    /// A [MeasurementUnit](https://developer.squareup.com/reference/square/objects/MeasurementUnit) that represents the unit of measure for the quantity.
    pub measurement_unit: Option<MeasurementUnitV20230925>,
    /// For non-integer quantities, represents the number of digits after the decimal point that are recorded for this quantity.
    ///
    /// For example, a precision of 1 allows quantities such as "1.0" and "1.1", but not "1.01".
    ///
    /// Min: 0. Max: 5.
    pub precision: Option<i32>,
    /// The catalog object ID referencing the [CatalogMeasurementUnit](https://developer.squareup.com/reference/square/objects/CatalogMeasurementUnit).
    ///
    /// This field is set when this is a catalog-backed measurement unit.
    pub catalog_object_id: Option<String>,
    /// The version of the catalog object that this measurement unit references.
    ///
    /// This field is set when this is a catalog-backed measurement unit.
    pub catalog_version: Option<i64>,
}
