//! ShippingFee

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingFee {
    /// The name for the shipping fee.
    pub name: Option<String>,
    /// The amount and currency for the shipping fee.
    pub charge: Money,
}
