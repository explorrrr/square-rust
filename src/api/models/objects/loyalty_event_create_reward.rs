//! LoyaltyEventCreateReward

use serde::{Deserialize, Serialize};

/// Provides metadata when the event type is CREATE_REWARD.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventCreateReward {
    /// The ID of the [loyalty program](https://developer.squareup.com/reference/square/objects/LoyaltyProgram).
    ///
    /// Min Length 1
    /// Max Length 36
    pub loyalty_program_id: String,
    /// The Square-assigned ID of the created loyalty reward. This field is returned only if the event source is LOYALTY_API.
    ///
    /// Max Length 36
    pub reward_id: Option<String>,
    /// The loyalty points used to create the reward.
    pub points: i32,
}
