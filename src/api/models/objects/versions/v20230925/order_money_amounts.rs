//! OrderMoneyAmounts

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;

/// A collection of various money amounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMoneyAmountsV20230925 {
    /// The total money.
    pub total_money: Option<MoneyV20230925>,
    /// The money associated with taxes.
    pub tax_money: Option<MoneyV20230925>,
    /// The money associated with discounts.
    pub discount_money: Option<MoneyV20230925>,
    /// The money associated with tips.
    pub tip_money: Option<MoneyV20230925>,
    /// The money associated with service charges.
    pub service_charge_money: Option<MoneyV20230925>,
}
