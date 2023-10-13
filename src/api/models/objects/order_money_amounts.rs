//! OrderMoneyAmounts

use serde::{Deserialize, Serialize};

/// A collection of various money amounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMoneyAmounts {
    /// The total money.
    pub total_money: Option<Money>,
    /// The money associated with taxes.
    pub tax_money: Option<Money>,
    /// The money associated with discounts.
    pub discount_money: Option<Money>,
    /// The money associated with tips.
    pub tip_money: Option<Money>,
    /// The money associated with service charges.
    pub service_charge_money: Option<Money>,
}
