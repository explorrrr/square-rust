//! LoyaltyEventAccumulatePromotionPoints

use serde::{Deserialize, Serialize};

/// Provides metadata when the event type is ACCUMULATE_PROMOTION_POINTS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventAccumulatePromotionPointsV20230925 {
    /// The Square-assigned ID of the [loyalty program](https://developer.squareup.com/reference/square/objects/LoyaltyProgram).
    ///
    /// Max Length 36
    pub loyalty_program_id: Option<String>,
    /// The Square-assigned ID of the [loyalty promotion](https://developer.squareup.com/reference/square/objects/LoyaltyProgram).
    ///
    /// Min Length 1
    /// Max Length 255
    pub loyalty_promotion_id: Option<String>,
    /// The number of points earned by the event.
    pub points: i32,
    /// The ID of the [order](https://developer.squareup.com/reference/square/objects/Order) for which the buyer earned the promotion points. Only applications that use the Orders API to process orders can trigger this event.
    ///
    /// Min Length 1
    pub order_id: String,
}
