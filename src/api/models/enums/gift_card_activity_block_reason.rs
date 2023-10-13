//! GiftCardActivityBlockReason Enum

use serde::{Deserialize, Serialize};

/// Indicates the reason for blocking a [gift card](https://developer.squareup.com/reference/square/objects/GiftCard).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GiftCardActivityBlockReason {
    /// The gift card is blocked because the buyer initiated a chargeback on the gift card purchase.
    ChargebackBlock,
}
