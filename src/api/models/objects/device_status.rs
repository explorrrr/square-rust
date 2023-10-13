//! DeviceStatus

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStatus {
    pub category: Option<DeviceStatusCategory>,
}
