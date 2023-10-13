//! DeviceComponentDetailsEthernetDetails

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceComponentDetailsEthernetDetailsBeta {
    /// A boolean to represent whether the Ethernet interface is currently active.
    pub active: Option<bool>,
    /// The string representation of the device's IPv4 address.
    pub ip_address_v4: Option<String>,
}
