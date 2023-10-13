//! LoyaltyProgramAccrualRule

use serde::{Deserialize, Serialize};

/// Represents an accrual rule, which defines how buyers can earn points from the base [loyalty program](https://developer.squareup.com/reference/square/objects/LoyaltyProgram).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyProgramAccrualRule {
    /// The type of the accrual rule that defines how buyers can earn points.
    pub accrual_type: LoyaltyProgramAccrualRuleType,
    /// The number of points that buyers earn based on the `accrual_type`.
    ///
    /// Min 1
    pub points: i32,
    /// Additional details for rules with the `VISIT` accrual type.
    pub visit_data: Option<LoyaltyProgramAccrualRuleVisitData>,
    /// Additional details for rules with the `SPEND` accrual type.
    pub spend_data: Option<LoyaltyProgramAccrualRuleSpendData>,
    /// Additional details for rules with the `ITEM_VARIATION` accrual type.
    pub item_variation_data: Option<LoyaltyProgramAccrualRuleItemVariationData>,
    /// Additional details for rules with the `CATEGORY` accrual type.
    pub category_data: Option<LoyaltyProgramAccrualRuleCategoryData>,
}
