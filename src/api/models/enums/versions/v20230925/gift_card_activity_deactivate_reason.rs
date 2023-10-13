//! GiftCardActivityDeactivateReason Enum

use serde::{Deserialize, Serialize};

/// Indicates the reason for deactivating a [gift card](https://developer.squareup.com/reference/square/objects/GiftCard).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GiftCardActivityDeactivateReasonV20230925 {
    /// The seller suspects suspicious activity.
    SuspiciousActivity,
    /// The gift card was deactivated for an unknown reason.
    /// This reason is read-only and cannot be used to create a DEACTIVATE activity using the Gift Card Activities API.
    UnknownReason,
    /// A chargeback on the gift card purchase (or the gift card load) was ruled in favor of the buyer.
    /// This reason is read-only and cannot be used to create a DEACTIVATE activity using the Gift Card Activities API.
    ChargebackDeactivate,
}
