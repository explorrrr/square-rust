//! OrderFulfillmentUpdatedObject

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderFulfillmentUpdatedObject {
    /// Information about the updated order fulfillment.
    pub order_fulfillment_updated: Option<OrderFulfillmentUpdated>,
}
