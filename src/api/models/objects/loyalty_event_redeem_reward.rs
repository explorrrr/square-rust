//! LoyaltyEventRedeemReward

use serde::{Deserialize, Serialize};

/// Provides metadata when the event type is REDEEM_REWARD.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventRedeemReward {
    /// The ID of the [loyalty program](https://developer.squareup.com/reference/square/objects/LoyaltyProgram).
    ///
    /// Min Length 1 Max Length 36
    pub loyalty_program_id: String,
    /// The ID of the redeemed [loyalty reward](https://developer.squareup.com/reference/square/objects/LoyaltyProgram). This field is returned only if the event source is LOYALTY_API.
    ///
    /// Max Length 36
    pub reward_id: Option<String>,
    /// The ID of the [order](https://developer.squareup.com/reference/square/objects/Order) that redeemed the reward. This field is returned only if the Orders API is used to process orders.
    pub order_id: Option<String>,
}
