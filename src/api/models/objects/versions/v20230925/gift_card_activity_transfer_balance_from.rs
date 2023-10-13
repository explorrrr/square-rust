//! GiftCardActivityTransferBalanceFrom


use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;

/// Represents details about a TRANSFER_BALANCE_FROM [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityTransferBalanceFromV20230925 {
    /// The ID of the gift card to which the specified amount was transferred.
    pub transfer_to_gift_card_id: String,
    /// The amount deducted from the gift card for the transfer. This value is a positive integer.
    pub amount_money: MoneyV20230925,
}
