//! TaxCalculationPhase Enum

use serde::{Deserialize, Serialize};

/// When to calculate the taxes due on a cart.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaxCalculationPhaseV20230925 {
    /// The fee is calculated based on the payment's subtotal.
    TaxSubtotalPhase,
    /// The fee is calculated based on the payment's total.
    TaxTotalPhase,
}
