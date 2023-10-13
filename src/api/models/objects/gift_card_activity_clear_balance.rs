//! GiftCardActivityClearBalance

use serde::{Deserialize, Serialize};

/// Represents details about a CLEAR_BALANCE [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityClearBalance {
    /// The reason the gift card balance was cleared.
    pub reason: GiftCardActivityClearBalanceReason,
}
