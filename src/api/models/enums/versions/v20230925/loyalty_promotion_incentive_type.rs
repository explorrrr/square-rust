//! LoyaltyPromotionIncentiveType Enum

use serde::{Deserialize, Serialize};

/// Indicates the type of points incentive for a [loyalty promotion](https://developer.squareup.com/reference/square/objects/LoyaltyPromotion), which is used to determine how buyers can earn points from the promotion.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoyaltyPromotionIncentiveTypeV20230925 {
    /// Multiply the number of points earned from the base loyalty program. For example, "Earn double points."
    PointsMultiplier,
    /// Add a specified number of points to those earned from the base loyalty program. For example, "Earn 10 additional points."
    PointsAddition,
}
