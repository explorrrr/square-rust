//! LoyaltyPromotionTriggerLimit

use serde::{Deserialize, Serialize};

/// Represents the number of times a buyer can earn points during a [loyalty promotion](https://developer.squareup.com/reference/square/objects/LoyaltyPromotion).
///
/// If this field is not set, buyers can trigger the promotion an unlimited number of times to earn points during the time that the promotion is available.
///
/// A purchase that is disqualified from earning points because of this limit might qualify for another active promotion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyPromotionTriggerLimit {
    /// The maximum number of times a buyer can trigger the promotion during the specified interval.
    ///
    /// Min: 1 Max: 30
    pub times: i32,
    /// The time period the limit applies to.
    pub interval: Option<LoyaltyPromotionTriggerLimitInterval>,
}
