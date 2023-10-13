//! FulfillmentType Enum

use serde::{Deserialize, Serialize};

/// The type of fulfillment.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FulfillmentType {
    /// A recipient to pick up the fulfillment from a physical location.
    Pickup,
    /// A shipping carrier to ship the fulfillment.
    Shipment,
    /// A courier to deliver the fulfillment.
    Delivery,
}
