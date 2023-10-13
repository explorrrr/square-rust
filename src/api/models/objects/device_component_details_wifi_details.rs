//! DeviceComponentDetailsWiFiDetails

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceComponentDetailsWiFiDetails {
    /// A boolean to represent whether the WiFI interface is currently active.
    pub active: Option<bool>,
    /// The name of the connected WIFI network.
    pub ssid: Option<String>,
    /// The string representation of the device's IPv4 address.
    pub ip_address_v4: Option<String>,
    /// The security protocol for a secure connection (e.g. WPA2). None provided if the connection is unsecured.
    pub secure_connection: Option<String>,
    /// A representation of signal strength of the WIFI network connection.
    pub signal_strength: Option<DeviceComponentDetailsMeasurement>,
}
