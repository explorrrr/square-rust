//! GiftCardActivityAdjustIncrement

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::gift_card_activity_adjust_increment_reason::GiftCardActivityAdjustIncrementReasonV20230925;

use super::money::MoneyV20230925;

/// Represents details about an ADJUST_INCREMENT [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityAdjustIncrementV20230925 {
    /// The amount added to the gift card balance. This value is a positive integer.
    pub amount_money: MoneyV20230925,
    /// The reason the gift card balance was adjusted.
    pub reason: GiftCardActivityAdjustIncrementReasonV20230925,
}
