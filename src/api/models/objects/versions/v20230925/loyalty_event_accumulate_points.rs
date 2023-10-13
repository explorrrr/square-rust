//! LoyaltyEventAccumulatePoints

use serde::{Deserialize, Serialize};

/// Provides metadata when the event type is ACCUMULATE_POINTS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventAccumulatePointsV20230925 {
    /// The ID of the [loyalty program](https://developer.squareup.com/reference/square/objects/LoyaltyProgram).
    ///
    /// Max Length 36
    pub loyalty_program_id: Option<String>,
    /// The number of points accumulated by the event.
    ///
    /// Min 1
    pub points: Option<i32>,
    /// The ID of the [order](https://developer.squareup.com/reference/square/objects/Order) for which the buyer accumulated the points.
    /// This field is returned only if the Orders API is used to process orders.
    pub order_id: Option<String>,
}
