//! DeviceComponentDetailsMeasurement

use serde::{Deserialize, Serialize};

/// A value qualified by unit of measure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceComponentDetailsMeasurement {
    pub value: Option<i32>,
}
