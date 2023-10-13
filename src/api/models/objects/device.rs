//! Device

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceBeta {
    /// Read only A synthetic identifier for the device. The identifier includes a standardized prefix and is otherwise an opaque id generated from key device fields.
    pub id: Option<String>,
    /// A collection of DeviceAttributes representing the device.
    pub attributes: DeviceAttributes,
    /// A list of components applicable to the device.
    pub components: Option<Vec<Component>>,
    /// Read only The current status of the device.
    pub status: Option<DeviceStatus>,
}
