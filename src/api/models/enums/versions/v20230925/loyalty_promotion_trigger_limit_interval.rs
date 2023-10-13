//! LoyaltyPromotionTriggerLimitInterval Enum

use serde::{Deserialize, Serialize};

/// Indicates the time period that the [trigger limit(https://developer.squareup.com/reference/square/objects/LoyaltyPromotionTriggerLimit)] applies to, which is used to determine the number of times a buyer can earn points for a [loyalty promotion](https://developer.squareup.com/reference/square/objects/LoyaltyPromotion).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoyaltyPromotionTriggerLimitIntervalV20230925 {
    /// The limit applies to the entire time that the promotion is active. For example, if times is set to 1 and time_period is set to ALL_TIME, a buyer can earn promotion points a maximum of one time during the promotion.
    AllTime,
    /// The limit applies per day, according to the available_time schedule specified for the promotion. For example, if the times field of the trigger limit is set to 1, a buyer can trigger the promotion a maximum of once per day.
    Day,
}
