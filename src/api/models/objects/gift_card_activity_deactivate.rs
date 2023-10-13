//! GiftCardActivityDeactivate

use serde::{Deserialize, Serialize};

/// Represents details about a DEACTIVATE [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityDeactivate {
    /// The reason the gift card was deactivated.
    pub reason: GiftCardActivityDeactivateReason,
}
