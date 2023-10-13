//! QuantityRatio

use serde::{Deserialize, Serialize};

/// A whole number or unreduced fractional ratio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantityRatioV20230925 {
    /// The whole or fractional quantity as the numerator.
    quantity: Option<i32>,
    /// The whole or fractional quantity as the denominator. In the case of fractional quantity this field is the denominator and quantity is the numerator. When unspecified, the value is 1. For example, when quantity=3 and quantity_donominator is unspecified, the quantity ratio is 3 or 3/1.
    quantity_denominator: Option<i32>,
}
