//! DeviceComponentDetailsNetworkInterfaceDetails

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceComponentDetailsNetworkInterfaceDetailsV20230925 {
    /// The string representation of the device's IPv4 address.
    pub ip_address_v4: Option<String>,
}
