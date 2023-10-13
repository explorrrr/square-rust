//! OrderUpdatedObject

use serde::{Deserialize, Serialize};

use super::order_updated::OrderUpdatedV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderUpdatedObjectV20230925 {
    /// Information about the updated order.
    pub order_updated: Option<OrderUpdatedV20230925>,
}
