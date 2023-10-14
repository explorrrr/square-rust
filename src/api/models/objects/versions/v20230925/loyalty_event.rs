//! LoyaltyEvent

use serde::{Deserialize, Serialize};

use super::{
    loyalty_event_accumulate_points::LoyaltyEventAccumulatePointsV20230925,
    loyalty_event_accumulate_promotion_points::LoyaltyEventAccumulatePromotionPointsV20230925,
    loyalty_event_adjust_points::LoyaltyEventAdjustPointsV20230925,
    loyalty_event_create_reward::LoyaltyEventCreateRewardV20230925,
    loyalty_event_delete_reward::LoyaltyEventDeleteRewardV20230925,
    loyalty_event_expire_points::LoyaltyEventExpirePointsV20230925, loyalty_event_other::LoyaltyEventOtherV20230925,
    loyalty_event_redeem_reward::LoyaltyEventRedeemRewardV20230925,
};
use crate::api::models::enums::versions::v20230925::loyalty_event_source::LoyaltyEventSourceV20230925;

/// Provides information about a loyalty event.
///
/// For more information, see [Search for Balance-Changing Loyalty Events](https://developer.squareup.com/docs/loyalty-api/loyalty-events).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventV20230925 {
    /// The Square-assigned ID of the loyalty event.
    ///
    /// Min Length 1
    pub id: String,
    /// The type of the loyalty event.
    pub r#type: String,
    /// The timestamp when the event was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// - UTC: 2020-01-26T02:25:34Z
    /// - Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    ///
    /// Min Length 1
    pub created_at: String,
    /// Provides metadata when the event type is ACCUMULATE_POINTS.
    pub accumulate_points: Option<LoyaltyEventAccumulatePointsV20230925>,
    /// Provides metadata when the event type is CREATE_REWARD.
    pub create_reward: Option<LoyaltyEventCreateRewardV20230925>,
    /// Provides metadata when the event type is REDEEM_REWARD.
    pub redeem_reward: Option<LoyaltyEventRedeemRewardV20230925>,
    /// Provides metadata when the event type is DELETE_REWARD.
    pub delete_reward: Option<LoyaltyEventDeleteRewardV20230925>,
    /// Provides metadata when the event type is ADJUST_POINTS.
    pub adjust_points: Option<LoyaltyEventAdjustPointsV20230925>,
    /// The ID of the [loyalty account](https://developer.squareup.com/reference/square/objects/LoyaltyAccount) associated with the event.
    ///
    /// Min Length 1 Max Length 36
    pub loyalty_account_id: String,
    /// The ID of the [location](https://developer.squareup.com/reference/square/objects/Location) where the event occurred.
    pub location_id: Option<String>,
    /// Defines whether the event was generated by the Square Point of Sale.
    pub source: LoyaltyEventSourceV20230925,
    /// Provides metadata when the event type is EXPIRE_POINTS.
    pub expire_points: Option<LoyaltyEventExpirePointsV20230925>,
    /// Provides metadata when the event type is OTHER.
    pub other_event: Option<LoyaltyEventOtherV20230925>,
    /// Provides metadata when the event type is ACCUMULATE_PROMOTION_POINTS.
    pub accumulate_promotion_points: Option<LoyaltyEventAccumulatePromotionPointsV20230925>,
}
