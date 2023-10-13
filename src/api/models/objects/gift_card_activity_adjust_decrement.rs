//! GiftCardActivityAdjustDecrement

use serde::{Deserialize, Serialize};

/// Represents details about an ADJUST_DECREMENT [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityAdjustDecrement {
    /// The amount deducted from the gift card balance. This value is a positive integer.
    pub amount_money: Money,
    /// The reason the gift card balance was adjusted.
    pub reason: GiftCardActivityAdjustDecrementReason,
}
