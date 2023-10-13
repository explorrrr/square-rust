//! OrderFulfillmentPickupDetailsScheduleType Enum

use serde::{Deserialize, Serialize};

/// The schedule type of the pickup fulfillment.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderFulfillmentPickupDetailsScheduleTypeV20230925 {
    /// Indicates that the fulfillment will be picked up at a scheduled pickup time.
    Scheduled,
    /// Indicates that the fulfillment will be picked up as soon as possible and should be prepared immediately.
    Asap,
}
