//! Component

use serde::{Deserialize, Serialize};

/// The wrapper object for the component entries of a given component type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    /// The type of this component. Each component type has expected properties expressed in a structured format within its corresponding *_details field.
    pub r#type: String,
    /// Read only Structured data for an Application, set for Components of type APPLICATION.
    pub application_details: Option<DeviceComponentDetailsApplicationDetails>,
    /// Read only Structured data for a CardReader, set for Components of type CARD_READER.
    pub card_reader_details: Option<DeviceComponentDetailsCardReaderDetails>,
    /// Read only Structured data for a Battery, set for Components of type BATTERY.
    pub battery_details: Option<DeviceComponentDetailsBatteryDetails>,
    /// Read only Structured data for a WiFi interface, set for Components of type WIFI.
    pub wifi_details: Option<DeviceComponentDetailsWiFiDetails>,
    /// Read only Structured data for an Ethernet interface, set for Components of type ETHERNET.
    pub ethernet_details: Option<DeviceComponentDetailsEthernetDetails>,
}
