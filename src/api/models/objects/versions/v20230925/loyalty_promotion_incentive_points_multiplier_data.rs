//! LoyaltyPromotionIncentivePointsMultiplierData

use serde::{Deserialize, Serialize};

/// Represents the metadata for a POINTS_MULTIPLIER type of [loyalty promotion incentive](https://developer.squareup.com/reference/square/objects/LoyaltyPromotionIncentive).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyPromotionIncentivePointsMultiplierDataV20230925 {
    /// The multiplier used to calculate the number of points earned each time the promotion is triggered, specified as a string representation of a decimal. Square supports multipliers up to 10x, with three point precision for decimal multipliers. For example, suppose a purchase qualifies for 4 points from the base loyalty program. If the purchase also qualifies for a POINTS_MULTIPLIER promotion incentive with a multiplier of "1.5", the buyer earns a total of 6 points (4 program points x 1.5 promotion multiplier = 6 points). Fractional points are dropped.
    ///
    /// One of the following is required when specifying a points multiplier:
    ///
    /// (Recommended) This multiplier field.
    /// The deprecated points_multiplier field. If provided in the request, Square also returns multiplier with the equivalent value.
    ///
    /// Max Length: 5
    pub multiplier: Option<String>,
}
