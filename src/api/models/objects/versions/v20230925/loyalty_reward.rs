//! LoyaltyReward

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::loyalty_reward_status::LoyaltyRewardStatusV20230925;

/// Represents a contract to redeem loyalty points for a [reward tier](https://developer.squareup.com/reference/square/objects/LoyaltyProgramRewardTier) discount.
///
/// Loyalty rewards can be in a `ISSUED`, `REDEEMED`, or `DELETED` state. For more information, see [Manage loyalty rewards](https://developer.squareup.com/docs/loyalty-api/loyalty-rewards).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyRewardV20230925 {
    /// The Square-assigned ID of the loyalty reward.
    ///
    /// Max Length: 36
    pub id: Option<String>,
    /// The status of a loyalty reward.
    pub status: Option<LoyaltyRewardStatusV20230925>,
    /// The Square-assigned ID of the [loyalty account](https://developer.squareup.com/reference/square/objects/LoyaltyAccount) to which the reward belongs.
    ///
    /// Min Length: 1 Max Length: 36
    pub loyalty_account_id: String,
    /// The Square-assigned ID of the [reward tier](https://developer.squareup.com/reference/square/objects/LoyaltyProgramRewardTier) used to create the reward.
    ///
    /// Min Length: 1 Max Length: 36
    pub reward_tier_id: String,
    /// The number of loyalty points used for the reward.
    ///
    /// Min: 1
    pub points: Option<i32>,
    /// The Square-assigned ID of the [order](https://developer.squareup.com/reference/square/objects/Order) to which the reward is attached.
    pub order_id: Option<String>,
    /// The timestamp when the reward was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// * UTC: 2020-01-26T02:25:34Z
    /// * Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// The timestamp when the reward was last updated, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// * UTC: 2020-01-26T02:25:34Z
    /// * Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
    /// The timestamp when the reward was redeemed, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// * UTC: 2020-01-26T02:25:34Z
    ///
    /// * Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub redeemed_at: Option<String>,
}
