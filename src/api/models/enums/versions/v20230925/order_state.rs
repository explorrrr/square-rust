//! OrderState Enum

use serde::{Deserialize, Serialize};

/// The state of the order.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStateV20230925 {
    /// Indicates that the order is open. Open orders can be updated.
    Open,
    /// Indicates that the order is completed. Completed orders are fully paid. This is a terminal state.
    Completed,
    /// Indicates that the order is canceled. Canceled orders are not paid. This is a terminal state.
    Canceled,
    /// Indicates that the order is in a draft state. Draft orders can be updated, but cannot be paid or fulfilled. For more information, see [Create Orders](https://developer.squareup.com/docs/orders-api/create-orders).
    Draft,
}
