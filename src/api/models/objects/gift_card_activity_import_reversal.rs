//! GiftCardActivityImportReversal

use serde::{Deserialize, Serialize};

/// Represents details about an IMPORT_REVERSAL [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityImportReversal {
    /// The amount of money cleared from the third-party gift card when the import was reversed.
    pub amount_money: Money,
}
