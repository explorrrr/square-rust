//! LoyaltyProgramAccrualRuleVisitData

use serde::{Deserialize, Serialize};

/// Represents additional data for rules with the VISIT accrual type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyProgramAccrualRuleVisitData {
    /// The minimum purchase required during the visit to quality for points.
    pub minimum_amount_money: Option<Money>,
    /// Indicates how taxes should be treated when calculating the purchase amount to determine whether the visit qualifies for points.
    /// This setting applies only if minimum_amount_money is specified.
    pub tax_mode: LoyaltyProgramAccrualRuleTaxMode,
}
