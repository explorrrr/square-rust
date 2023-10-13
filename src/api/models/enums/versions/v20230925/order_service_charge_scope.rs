//! OrderServiceChargeScope Enum

use serde::{Deserialize, Serialize};

/// Indicates whether this is a line-item or order-level apportioned service charge.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderServiceChargeScopeV20230925 {
    /// Used for reporting only. The original transaction service charge scope is currently not supported by the API.
    OtherServiceChargeScope,
    /// The service charge should be applied to only line items specified by OrderLineItemAppliedServiceCharge reference records.
    LineItem,
    /// The service charge should be applied to the entire order.
    Order,
}
