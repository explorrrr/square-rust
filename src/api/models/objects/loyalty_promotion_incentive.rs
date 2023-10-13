//! LoyaltyPromotionIncentive

use serde::{Deserialize, Serialize};

/// Represents how points for a [loyalty promotion](https://developer.squareup.com/reference/square/objects/LoyaltyPromotion) are calculated, either by multiplying the points earned from the base program or by adding a specified number of points to the points earned from the base program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyPromotionIncentive {
    /// The type of points incentive.
    pub r#type: LoyaltyPromotionIncentiveType,
    /// Additional data for a POINTS_MULTIPLIER incentive type.
    pub points_multiplier_data: Option<LoyaltyPromotionIncentivePointsMultiplierData>,
    /// Additional data for a POINTS_ADDITION incentive type.
    pub points_addition_data: Option<LoyaltyPromotionIncentivePointsAdditionData>,
}
