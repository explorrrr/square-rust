//! GiftCardActivityTransferBalanceTo

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;

/// Represents details about a TRANSFER_BALANCE_TO [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityTransferBalanceToV20230925 {
    /// The ID of the gift card from which the specified amount was transferred.
    pub transfer_from_gift_card_id: String,
    /// The amount added to the gift card balance for the transfer. This value is a positive integer.
    pub amount_money: MoneyV20230925,
}
