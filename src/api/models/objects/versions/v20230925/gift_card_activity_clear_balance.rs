//! GiftCardActivityClearBalance

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::gift_card_activity_clear_balance_reason::GiftCardActivityClearBalanceReasonV20230925;

/// Represents details about a CLEAR_BALANCE [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityClearBalanceV20230925 {
    /// The reason the gift card balance was cleared.
    pub reason: GiftCardActivityClearBalanceReasonV20230925,
}
