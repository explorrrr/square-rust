//! ShippingFee

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingFeeV20230925 {
    /// The name for the shipping fee.
    pub name: Option<String>,
    /// The amount and currency for the shipping fee.
    pub charge: MoneyV20230925,
}
