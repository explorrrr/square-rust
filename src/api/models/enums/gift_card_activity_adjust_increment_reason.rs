//! GiftCardActivityAdjustIncrementReason Enum

use serde::{Deserialize, Serialize};

/// Indicates the reason for adding money to a [gift card](https://developer.squareup.com/reference/square/objects/GiftCard).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GiftCardActivityAdjustIncrementReason {
    /// The seller gifted a complimentary gift card balance increase.
    Complimentary,
    /// The seller increased the gift card balance to accommodate support issues.
    SupportIssue,
    /// The transaction is voided.
    TransactionVoided,
}
