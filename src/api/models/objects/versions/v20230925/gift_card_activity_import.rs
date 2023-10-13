//! GiftCardActivityImport

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;

/// Represents details about an IMPORT [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityImportV20230925 {
    /// The balance amount on the imported gift card.
    pub amount_money: MoneyV20230925,
}
