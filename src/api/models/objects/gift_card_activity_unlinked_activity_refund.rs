//! GiftCardActivityUnlinkedActivityRefund

use serde::{Deserialize, Serialize};

/// Represents details about an UNLINKED_ACTIVITY_REFUND [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityUnlinkedActivityRefund {
    /// The amount added to the gift card for the refund. This value is a positive integer.
    pub amount_money: Money,
    /// A client-specified ID that associates the gift card activity with an entity in another system.
    pub reference_id: Option<String>,
    /// Read only The ID of the refunded payment. This field is not used starting in Square version 2022-06-16.
    pub payment_id: Option<String>,
}
