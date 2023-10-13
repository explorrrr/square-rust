//! GiftCardActivityActivate

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;

/// Represents details about an ACTIVATE [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityActivateV20230925 {
    /// The amount added to the gift card. This value is a positive integer.
    ///
    /// Applications that use a custom order processing system must specify this amount in the [CreateGiftCardActivity](https://developer.squareup.com/reference/square/giftcardactivities-api/create-gift-card-activity) request.
    pub amount_money: Option<MoneyV20230925>,
    /// The ID of the [order](https://developer.squareup.com/reference/square/objects/Order) that contains the GIFT_CARD line item.
    ///
    /// Applications that use the Square Orders API to process orders must specify the order ID [CreateGiftCardActivity](https://developer.squareup.com/reference/square/giftcardactivities-api/create-gift-card-activity) request.
    pub order_id: Option<String>,
    /// The UID of the GIFT_CARD line item in the order that represents the gift card purchase.
    ///
    /// Applications that use the Square Orders API to process orders must specify the line item UID in the [CreateGiftCardActivity](https://developer.squareup.com/reference/square/giftcardactivities-api/create-gift-card-activity) request.
    pub line_item_uid: Option<String>,
    /// A client-specified ID that associates the gift card activity with an entity in another system.
    ///
    /// Applications that use a custom order processing system can use this field to track information related to an order or payment.
    pub reference_id: Option<String>,
    /// The payment instrument IDs used to process the gift card purchase, such as a credit card ID or bank account ID.
    ///
    /// Applications that use a custom order processing system must specify payment instrument IDs in the [CreateGiftCardActivity](https://developer.squareup.com/reference/square/giftcardactivities-api/create-gift-card-activity) request. Square uses this information to perform compliance checks.
    ///
    /// For applications that use the Square Orders API to process payments, Square has the necessary instrument IDs to perform compliance checks.
    pub buyer_payment_instrument_ids: Option<Vec<String>>,
}
