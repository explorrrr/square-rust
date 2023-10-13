//! OrderUpdatedObject

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderUpdatedObject {
    /// Information about the updated order.
    pub order_updated: Option<OrderUpdated>,
}
