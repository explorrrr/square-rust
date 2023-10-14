//! LoyaltyProgramAccrualRuleVisitData

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;
use crate::api::models::enums::versions::v20230925::loyalty_program_accrual_rule_tax_mode::LoyaltyProgramAccrualRuleTaxModeV20230925;

/// Represents additional data for rules with the VISIT accrual type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyProgramAccrualRuleVisitDataV20230925 {
    /// The minimum purchase required during the visit to quality for points.
    pub minimum_amount_money: Option<MoneyV20230925>,
    /// Indicates how taxes should be treated when calculating the purchase amount to determine whether the visit qualifies for points.
    /// This setting applies only if minimum_amount_money is specified.
    pub tax_mode: LoyaltyProgramAccrualRuleTaxModeV20230925,
}
