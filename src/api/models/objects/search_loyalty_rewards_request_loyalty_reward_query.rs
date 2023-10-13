//! SearchLoyaltyRewardsRequestLoyaltyRewardQuery

use serde::{Deserialize, Serialize};

/// The set of search requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchLoyaltyRewardsRequestLoyaltyRewardQuery {
    /// The ID of the [loyalty account](https://developer.squareup.com/reference/square/objects/LoyaltyAccount) to which the loyalty reward belongs.
    ///
    /// Min Length 1
    pub loyalty_account_id: String,
    /// The status of the loyalty reward.
    pub status: Option<LoyaltyRewardStatus>,
}
