//! LoyaltyPromotionIncentivePointsAdditionData

use serde::{Deserialize, Serialize};

/// Represents the metadata for a POINTS_ADDITION type of [loyalty promotion incentive](https://developer.squareup.com/reference/square/objects/LoyaltyPromotionIncentive).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyPromotionIncentivePointsAdditionDataV20230925 {
    /// The number of additional points to earn each time the promotion is triggered. For example, suppose a purchase qualifies for 5 points from the base loyalty program. If the purchase also qualifies for a POINTS_ADDITION promotion incentive with a points_addition of 3, the buyer earns a total of 8 points (5 program points + 3 promotion points = 8 points).
    pub points_addition: i32,
}
