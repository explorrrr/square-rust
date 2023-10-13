//! GiftCard

use serde::{Deserialize, Serialize};

/// Represents a Square gift card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCard {
    /// The Square-assigned ID of the gift card.
    pub id: Option<String>,
    /// The gift card type.
    pub r#type: GiftCardType,
    /// The source that generated the gift card account number (GAN). The default value is SQUARE.
    pub gan_source: Option<GiftCardGANSource>,
    /// The current gift card state.
    pub state: Option<GiftCardStatus>,
    /// The current gift card balance. This balance is always greater than or equal to zero.
    pub balance_money: Option<Money>,
    /// The gift card account number (GAN). Buyers can use the GAN to make purchases or check the gift card balance.
    pub gan: Option<String>,
    /// The timestamp when the gift card was created, in RFC 3339 format. In the case of a digital gift card, it is the time when you create a card (using the Square Point of Sale application, Seller Dashboard, or Gift Cards API).
    /// In the case of a plastic gift card, it is the time when Square associates the card with the seller at the time of activation.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// The IDs of the [customer](https://developer.squareup.com/reference/square/objects/Customer) profiles to whom this gift card is linked.
    pub customer_ids: Option<Vec<String>>,
}
