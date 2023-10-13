//! SearchLoyaltyRewardsRequestLoyaltyRewardQuery

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::loyalty_reward_status::LoyaltyRewardStatusV20230925;

/// The set of search requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchLoyaltyRewardsRequestLoyaltyRewardQueryV20230925 {
    /// The ID of the [loyalty account](https://developer.squareup.com/reference/square/objects/LoyaltyAccount) to which the loyalty reward belongs.
    ///
    /// Min Length 1
    pub loyalty_account_id: String,
    /// The status of the loyalty reward.
    pub status: Option<LoyaltyRewardStatusV20230925>,
}
