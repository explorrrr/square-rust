//! LoyaltyEventOrderFilter

use serde::{Deserialize, Serialize};

/// Filter events by the order associated with the event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventOrderFilter {
    /// The ID of the [order](https://developer.squareup.com/reference/square/objects/Order) associated with the event.
    ///
    /// Min Length 1
    pub order_id: String,
}
