//! GiftCardActivityAdjustDecrementReason Enum

use serde::{Deserialize, Serialize};

/// Indicates the reason for deducting money from a [gift card](https://developer.squareup.com/reference/square/objects/GiftCard).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GiftCardActivityAdjustDecrementReasonV20230925 {
    /// The balance was decreased because the seller detected suspicious or fraudulent activity on the gift card.
    SuspiciousActivity,
    /// The balance was decreased to reverse an unintentional balance increase.
    BalanceAccidentallyIncreased,
    /// The balance was decreased to accommodate support issues.
    SupportIssue,
    /// The balance was decreased because the order used to purchase or reload the gift card was refunded.
    PurchaseWasRefunded,
}
