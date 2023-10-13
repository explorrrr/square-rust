//! DeviceCodeStatus enum

use serde::{Deserialize, Serialize};

/// DeviceCode.Status enum.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceCodeStatus {
    /// The status cannot be determined or does not exist.
    Unknown,
    /// The device code is just created and unpaired.
    Unpaired,
    /// The device code has been signed in and paired to a device.
    Paired,
    /// The device code was unpaired and expired before it was paired.
    Expired,
}
