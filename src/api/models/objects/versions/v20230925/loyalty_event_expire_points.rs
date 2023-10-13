//! LoyaltyEventExpirePoints

use serde::{Deserialize, Serialize};

/// Provides metadata when the event type is EXPIRE_POINTS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventExpirePointsV20230925 {
    /// The Square-assigned ID of the [loyalty program](https://developer.squareup.com/reference/square/objects/LoyaltyProgram).
    ///
    /// Min Length 1 Max Length 36
    pub loyalty_program_id: String,
    /// The number of points expired.
    pub points: i32,
}
