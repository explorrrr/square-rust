//! OrderReward

use serde::{Deserialize, Serialize};

/// Represents a reward that can be applied to an order if the necessary reward tier criteria are met.
///
/// Rewards are created through the Loyalty API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderReward {
    /// The identifier of the reward.
    ///
    /// Min Length 1
    pub id: String,
    /// The identifier of the reward tier corresponding to this reward.
    ///
    /// Min Length 1
    pub reward_tier_id: String,
}
