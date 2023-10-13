//! DeviceAttributesDeviceType enum

use serde::{Deserialize, Serialize};

/// An enum identifier of the device type.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceAttributesDeviceTypeV20230925 {
    Terminal,
}
