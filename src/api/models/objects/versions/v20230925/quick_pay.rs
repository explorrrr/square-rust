//! QuickPay

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;

/// Describes an ad hoc item and price to generate a quick pay checkout link.
///
/// For more information, see [Quick Pay Checkout](https://developer.squareup.com/docs/checkout-api/quick-pay-checkout).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickPayV20230925 {
    /// The ad hoc item name. In the resulting Order, this name appears as the line item name.
    ///
    /// Min Length: 1. Max Length: 255.
    pub name: String,
    /// The price of the item.
    pub price_money: MoneyV20230925,
    /// The ID of the business location the checkout is associated with.
    pub location_id: String,
}
