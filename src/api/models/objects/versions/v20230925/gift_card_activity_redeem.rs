//! GiftCardActivityRedeem

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;
use crate::api::models::enums::versions::v20230925::gift_card_activity_redeem_status::GiftCardActivityRedeemStatusV20230925;

/// Represents details about a REDEEM [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityRedeemV20230925 {
    /// The amount deducted from the gift card for the redemption. This value is a positive integer.
    ///
    /// Applications that use a custom payment processing system must specify this amount in the [CreateGiftCardActivity](https://developer.squareup.com/reference/square/giftcardactivities-api/create-gift-card-activity) request.
    pub amount_money: MoneyV20230925,
    /// Read only The ID of the payment that represents the gift card redemption. Square populates this field if the payment was processed by Square.
    pub payment_id: Option<String>,
    /// A client-specified ID that associates the gift card activity with an entity in another system.
    ///
    /// Applications that use a custom payment processing system can use this field to track information related to an order or payment.
    pub reference_id: Option<String>,
    /// Read only The status of the gift card redemption. Gift cards redeemed from Square Point of Sale or the Square Seller Dashboard use a two-state process: PENDING to COMPLETED or PENDING to CANCELED. Gift cards redeemed using the Gift Card Activities API always have a COMPLETED status.
    pub status: Option<GiftCardActivityRedeemStatusV20230925>,
}
