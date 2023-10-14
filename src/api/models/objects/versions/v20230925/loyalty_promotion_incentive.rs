//! LoyaltyPromotionIncentive

use serde::{Deserialize, Serialize};

use super::{
    loyalty_promotion_incentive_points_addition_data::LoyaltyPromotionIncentivePointsAdditionDataV20230925,
    loyalty_promotion_incentive_points_multiplier_data::LoyaltyPromotionIncentivePointsMultiplierDataV20230925,
};
use crate::api::models::enums::versions::v20230925::loyalty_promotion_incentive_type::LoyaltyPromotionIncentiveTypeV20230925;

/// Represents how points for a [loyalty promotion](https://developer.squareup.com/reference/square/objects/LoyaltyPromotion) are calculated, either by multiplying the points earned from the base program or by adding a specified number of points to the points earned from the base program.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyPromotionIncentiveV20230925 {
    /// The type of points incentive.
    pub r#type: LoyaltyPromotionIncentiveTypeV20230925,
    /// Additional data for a POINTS_MULTIPLIER incentive type.
    pub points_multiplier_data: Option<LoyaltyPromotionIncentivePointsMultiplierDataV20230925>,
    /// Additional data for a POINTS_ADDITION incentive type.
    pub points_addition_data: Option<LoyaltyPromotionIncentivePointsAdditionDataV20230925>,
}
