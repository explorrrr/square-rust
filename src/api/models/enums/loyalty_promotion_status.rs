//! LoyaltyPromotionStatus Enum

use serde::{Deserialize, Serialize};

/// Indicates the status of a [loyalty promotion](https://developer.squareup.com/reference/square/objects/LoyaltyPromotion).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoyaltyPromotionStatus {
    /// The loyalty promotion is currently active. Buyers can earn points for purchases that meet the promotion conditions, such as the promotion's available_time.
    Active,
    /// The loyalty promotion has ended because the specified end_date was reached. ENDED is a terminal status.
    Ended,
    /// The loyalty promotion was canceled. CANCELED is a terminal status.
    Canceled,
    /// The loyalty promotion is scheduled to start in the future. Square changes the promotion status to ACTIVE when the start_date is reached.
    Scheduled,
}
