//! DeviceAttributes

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAttributes {
    /// The device type.
    pub r#type: DeviceAttributesDeviceType,
    /// The maker of the device.
    pub manufacturer: String,
    /// The specific model of the device.
    pub model: Option<String>,
    /// A seller-specified name for the device.
    pub name: Option<String>,
    /// The manufacturer-supplied identifier for the device (where available). In many cases, this identifier will be a serial number.
    pub manufacturers_id: Option<String>,
    /// The RFC 3339-formatted value of the most recent update to the device information. (Could represent any field update on the device.)
    pub updated_at: Option<String>,
    /// The current version of software installed on the device.
    pub version: Option<String>,
    /// The merchant_token identifying the merchant controlling the device.
    pub merchant_token: Option<String>,
}
