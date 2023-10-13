//! OrderCreatedObject

use serde::{Deserialize, Serialize};

use super::order_created::OrderCreatedV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreatedObjectV20230925 {
    /// Information about the created order.
    pub order_created: Option<OrderCreatedV20230925>,
}
