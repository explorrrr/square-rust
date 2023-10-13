//! GiftCardActivityUnblockReason Enum

use serde::{Deserialize, Serialize};

/// Indicates the reason for unblocking a [gift card](https://developer.squareup.com/reference/square/objects/GiftCard).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GiftCardActivityUnblockReason {
    /// The gift card is unblocked because a chargeback was ruled in favor of the seller.
    ChargebackUnblock,
}
