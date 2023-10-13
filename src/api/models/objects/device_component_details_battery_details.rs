//! DeviceComponentDetailsBatteryDetails

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceComponentDetailsBatteryDetails {
    /// The battery charge percentage as displayed on the device.
    pub visible_percent: Option<i32>,
    /// The status of external_power.
    pub external_power: Option<DeviceComponentDetailsExternalPower>,
}
