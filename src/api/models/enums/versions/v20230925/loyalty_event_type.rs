//! LoyaltyEventType Enum

use serde::{Deserialize, Serialize};

/// The type of the loyalty event.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoyaltyEventTypeV20230925 {
    /// Points are added to a loyalty account for a purchase that qualified for points based on an [accrual rule](https://developer.squareup.com/reference/square/objects/LoyaltyProgramAccrualRule).
    AccumulatePoints,
    /// A [loyalty reward](https://developer.squareup.com/reference/square/objects/LoyaltyReward) is created.
    CreateReward,
    /// A loyalty reward is redeemed.
    RedeemReward,
    /// A loyalty reward is deleted.
    DeleteReward,
    /// Loyalty points are manually adjusted.
    AdjustPoints,
    /// Loyalty points are expired according to the expiration policy of the loyalty program.
    ExpirePoints,
    /// Some other loyalty event occurred.
    Other,
    /// Points are added to a loyalty account for a purchase that qualified for a [loyalty promotion](https://developer.squareup.com/reference/square/objects/LoyaltyPromotion).
    AccumulatePromotionPoints,
}
