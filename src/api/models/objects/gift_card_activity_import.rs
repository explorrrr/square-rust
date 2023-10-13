//! GiftCardActivityImport

use serde::{Deserialize, Serialize};

/// Represents details about an IMPORT [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityImport {
    /// The balance amount on the imported gift card.
    pub amount_money: Money,
}
