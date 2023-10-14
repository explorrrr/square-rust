//! Component

use serde::{Deserialize, Serialize};

use super::{
    device_component_details_application_details::DeviceComponentDetailsApplicationDetailsV20230925,
    device_component_details_battery_details::DeviceComponentDetailsBatteryDetailsV20230925,
    device_component_details_card_reader_details::DeviceComponentDetailsCardReaderDetailsV20230925,
    device_component_details_ethernet_details::DeviceComponentDetailsEthernetDetailsV20230925,
    device_component_details_wifi_details::DeviceComponentDetailsWiFiDetailsV20230925,
};

/// The wrapper object for the component entries of a given component type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentV20230925 {
    /// The type of this component. Each component type has expected properties expressed in a structured format within its corresponding *_details field.
    pub r#type: String,
    /// Read only Structured data for an Application, set for Components of type APPLICATION.
    pub application_details: Option<DeviceComponentDetailsApplicationDetailsV20230925>,
    /// Read only Structured data for a CardReader, set for Components of type CARD_READER.
    pub card_reader_details: Option<DeviceComponentDetailsCardReaderDetailsV20230925>,
    /// Read only Structured data for a Battery, set for Components of type BATTERY.
    pub battery_details: Option<DeviceComponentDetailsBatteryDetailsV20230925>,
    /// Read only Structured data for a WiFi interface, set for Components of type WIFI.
    pub wifi_details: Option<DeviceComponentDetailsWiFiDetailsV20230925>,
    /// Read only Structured data for an Ethernet interface, set for Components of type ETHERNET.
    pub ethernet_details: Option<DeviceComponentDetailsEthernetDetailsV20230925>,
}
