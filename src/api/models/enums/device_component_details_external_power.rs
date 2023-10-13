//! DeviceComponentDetailsExternalPower Enum

use serde::{Deserialize, Serialize};

/// An enum for ExternalPower.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceComponentDetailsExternalPower {
    /// Plugged in and charging.
    AvailableCharging,
    /// Fully charged.
    AvailableNotInUse,
    /// On battery power.
    Unavailable,
    /// Not providing enough power for the device.
    AvailableInsufficient,
}
