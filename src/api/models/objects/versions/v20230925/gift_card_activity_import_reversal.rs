//! GiftCardActivityImportReversal

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;

/// Represents details about an IMPORT_REVERSAL [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityImportReversalV20230925 {
    /// The amount of money cleared from the third-party gift card when the import was reversed.
    pub amount_money: MoneyV20230925,
}
