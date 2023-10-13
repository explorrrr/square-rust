//! LoyaltyProgramAccrualRuleTaxMode Enum

use serde::{Deserialize, Serialize};

/// Indicates how taxes should be treated when calculating the purchase amount used for loyalty points accrual.
/// This setting applies only to SPEND accrual rules or VISIT accrual rules that have a minimum spend requirement.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoyaltyProgramAccrualRuleTaxMode {
    /// Exclude taxes from the purchase amount used for loyalty points accrual.
    BeforeTax,
    /// Include taxes in the purchase amount used for loyalty points accrual.
    AfterTax,
}
