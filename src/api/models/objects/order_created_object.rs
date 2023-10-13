//! OrderCreatedObject

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreatedObject {
    /// Information about the created order.
    pub order_created: Option<OrderCreated>,
}
