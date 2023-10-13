//! OrderRoundingAdjustment

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;

/// A rounding adjustment of the money being returned.
///
/// Commonly used to apply cash rounding when the minimum unit of the account is smaller than the lowest physical denomination of the currency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRoundingAdjustmentV20230925 {
    /// A unique ID that identifies the rounding adjustment only within this order.
    ///
    /// Max Length 60
    pub uid: Option<String>,
    /// The name of the rounding adjustment from the original sale order.
    pub name: Option<String>,
    /// The actual rounding adjustment amount.
    pub amount_money: Option<MoneyV20230925>,
}
