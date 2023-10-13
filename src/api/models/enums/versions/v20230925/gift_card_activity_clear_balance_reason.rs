//! GiftCardActivityClearBalanceReason Enum

use serde::{Deserialize, Serialize};

/// Indicates the reason for clearing the balance of a [gift card](https://developer.squareup.com/reference/square/objects/GiftCard).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GiftCardActivityClearBalanceReasonV20230925 {
    /// The seller suspects suspicious activity.
    SuspiciousActivity,
    /// The seller cleared the balance to reuse the gift card.
    ReuseGiftcard,
    /// The gift card balance was cleared for an unknown reason.
    /// This reason is read-only and cannot be used to create a CLEAR_BALANCE activity using the Gift Card Activities API.
    UnknownReason,
}
