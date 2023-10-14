//! LoyaltyProgramAccrualRule

use serde::{Deserialize, Serialize};

use super::{
    loyalty_program_accrual_rule_category_data::LoyaltyProgramAccrualRuleCategoryDataV20230925,
    loyalty_program_accrual_rule_item_variation_data::LoyaltyProgramAccrualRuleItemVariationDataV20230925,
    loyalty_program_accrual_rule_spend_data::LoyaltyProgramAccrualRuleSpendDataV20230925,
    loyalty_program_accrual_rule_visit_data::LoyaltyProgramAccrualRuleVisitDataV20230925,
};
use crate::api::models::enums::versions::v20230925::loyalty_program_accrual_rule_type::LoyaltyProgramAccrualRuleTypeV20230925;

/// Represents an accrual rule, which defines how buyers can earn points from the base [loyalty program](https://developer.squareup.com/reference/square/objects/LoyaltyProgram).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyProgramAccrualRuleV20230925 {
    /// The type of the accrual rule that defines how buyers can earn points.
    pub accrual_type: LoyaltyProgramAccrualRuleTypeV20230925,
    /// The number of points that buyers earn based on the `accrual_type`.
    ///
    /// Min 1
    pub points: i32,
    /// Additional details for rules with the `VISIT` accrual type.
    pub visit_data: Option<LoyaltyProgramAccrualRuleVisitDataV20230925>,
    /// Additional details for rules with the `SPEND` accrual type.
    pub spend_data: Option<LoyaltyProgramAccrualRuleSpendDataV20230925>,
    /// Additional details for rules with the `ITEM_VARIATION` accrual type.
    pub item_variation_data: Option<LoyaltyProgramAccrualRuleItemVariationDataV20230925>,
    /// Additional details for rules with the `CATEGORY` accrual type.
    pub category_data: Option<LoyaltyProgramAccrualRuleCategoryDataV20230925>,
}
