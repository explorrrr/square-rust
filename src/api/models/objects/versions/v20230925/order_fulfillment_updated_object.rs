//! OrderFulfillmentUpdatedObject

use serde::{Deserialize, Serialize};

use super::order_fulfillment_updated::OrderFulfillmentUpdatedV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderFulfillmentUpdatedObjectV20230925 {
    /// Information about the updated order fulfillment.
    pub order_fulfillment_updated: Option<OrderFulfillmentUpdatedV20230925>,
}
