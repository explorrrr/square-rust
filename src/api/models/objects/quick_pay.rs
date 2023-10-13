//! QuickPay

use serde::{Deserialize, Serialize};

/// Describes an ad hoc item and price to generate a quick pay checkout link.
///
/// For more information, see [Quick Pay Checkout](https://developer.squareup.com/docs/checkout-api/quick-pay-checkout).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickPay {
    /// The ad hoc item name. In the resulting Order, this name appears as the line item name.
    ///
    /// Min Length: 1. Max Length: 255.
    pub name: String,
    /// The price of the item.
    pub price_money: Money,
    /// The ID of the business location the checkout is associated with.
    pub location_id: String,
}
