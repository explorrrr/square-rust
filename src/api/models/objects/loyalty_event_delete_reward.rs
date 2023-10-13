//! LoyaltyEventDeleteReward

use serde::{Deserialize, Serialize};

/// Provides metadata when the event type is DELETE_REWARD.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventDeleteReward {
    /// The ID of the [loyalty program](https://developer.squareup.com/reference/square/objects/LoyaltyProgram).
    ///
    /// Min Length 1
    /// Max Length 36
    pub loyalty_program_id: String,
    /// The ID of the deleted loyalty reward. This field is returned only if the event source is LOYALTY_API.
    ///
    /// Max Length 36
    pub reward_id: Option<String>,
    /// The number of points returned to the loyalty account.
    pub points: i32,
}
