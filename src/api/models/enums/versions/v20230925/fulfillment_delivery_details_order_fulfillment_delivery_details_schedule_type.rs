//! FulfillmentDeliveryDetailsOrderFulfillmentDeliveryDetailsScheduleType Enum

use serde::{Deserialize, Serialize};

/// Indicates the type of pickup fulfillment.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FulfillmentDeliveryDetailsOrderFulfillmentDeliveryDetailsScheduleTypeV20230925 {
    /// Indicates the fulfillment to deliver at a scheduled deliver time.
    Scheduled,
    /// Indicates that the fulfillment to deliver as soon as possible and should be prepared immediately.
    Asap,
}
