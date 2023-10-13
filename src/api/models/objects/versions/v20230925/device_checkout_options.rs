//! DeviceCheckoutOptions

use serde::{Deserialize, Serialize};

use super::tip_settings::TipSettingsV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCheckoutOptionsV20230925 {
    /// The unique ID of the device intended for this TerminalCheckout. A list of DeviceCode objects can be retrieved from the /v2/devices/codes endpoint. Match a DeviceCode.device_id value with device_id to get the associated device code.
    pub device_id: String,
    /// Instructs the device to skip the receipt screen. Defaults to false.
    pub skip_receipt_screen: Option<bool>,
    /// Indicates that signature collection is desired during checkout. Defaults to false.
    pub collect_signature: Option<bool>,
    /// Tip-specific settings.
    pub tip_settings: Option<TipSettingsV20230925>,
    /// Show the itemization screen prior to taking a payment. This field is only meaningful when the checkout includes an order ID. Defaults to true.
    pub show_itemized_cart: Option<bool>,
}
