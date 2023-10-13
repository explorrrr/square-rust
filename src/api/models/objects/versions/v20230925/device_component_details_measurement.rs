//! DeviceComponentDetailsMeasurement

use serde::{Deserialize, Serialize};

/// A value qualified by unit of measure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceComponentDetailsMeasurementV20230925 {
    pub value: Option<i32>,
}
