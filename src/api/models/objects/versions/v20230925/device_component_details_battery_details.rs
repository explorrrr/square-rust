//! DeviceComponentDetailsBatteryDetails

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::device_component_details_external_power::DeviceComponentDetailsExternalPowerV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceComponentDetailsBatteryDetailsV20230925 {
    /// The battery charge percentage as displayed on the device.
    pub visible_percent: Option<i32>,
    /// The status of external_power.
    pub external_power: Option<DeviceComponentDetailsExternalPowerV20230925>,
}
