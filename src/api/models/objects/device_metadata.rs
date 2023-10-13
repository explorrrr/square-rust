//! DeviceMetadata

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceMetadata {
    /// The Terminal’s remaining battery percentage, between 1-100.
    pub battery_percentage: Option<String>,
    /// The current charging state of the Terminal. Options: CHARGING, NOT_CHARGING
    pub charging_state: Option<String>,
    /// The ID of the Square seller business location associated with the Terminal.
    pub location_id: Option<String>,
    /// The ID of the Square merchant account that is currently signed-in to the Terminal.
    pub merchant_id: Option<String>,
    /// The Terminal’s current network connection type. Options: WIFI, ETHERNET
    pub network_connection_type: Option<String>,
    /// The country in which the Terminal is authorized to take payments.
    pub payment_region: Option<String>,
    /// The unique identifier assigned to the Terminal, which can be found on the lower back of the device.
    pub serial_number: Option<String>,
    /// The current version of the Terminal’s operating system.
    pub os_version: Option<String>,
    /// The current version of the application running on the Terminal.
    pub app_version: Option<String>,
    /// The name of the Wi-Fi network to which the Terminal is connected.
    pub wifi_network_name: Option<String>,
    /// The signal strength of the Wi-FI network connection. Options: POOR, FAIR, GOOD, EXCELLENT
    pub wifi_network_strength: Option<String>,
    /// The IP address of the Terminal.
    pub ip_address: Option<String>,
}
