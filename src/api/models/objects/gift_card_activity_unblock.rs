//! GiftCardActivityUnblock

use serde::{Deserialize, Serialize};

/// Represents details about an UNBLOCK [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityUnblock {
    /// The reason the gift card was unblocked.
    pub reason: GiftCardActivityUnblockReason,
}
