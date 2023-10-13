//! GiftCardActivityBlock

use serde::{Deserialize, Serialize};

/// Represents details about a BLOCK [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityBlock {
    /// The reason the gift card was blocked.
    pub reason: GiftCardActivityBlockReason,
}
