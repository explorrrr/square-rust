//! DeviceDetails

use serde::{Deserialize, Serialize};

/// Details about the device that took the payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDetails {
    /// The Square-issued ID of the device.
    /// Max Length: 255
    pub device_id: Option<String>,
    /// The Square-issued installation ID for the device.
    /// Max Length: 255
    pub device_installation_id: Option<String>,
    /// The name of the device set by the seller.
    /// Max Length: 255
    pub device_name: Option<String>,
}
