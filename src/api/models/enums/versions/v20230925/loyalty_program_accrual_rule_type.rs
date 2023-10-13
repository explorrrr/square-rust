//! LoyaltyProgramAccrualRuleType Enum

use serde::{Deserialize, Serialize};

/// The type of the accrual rule that defines how buyers can earn points.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoyaltyProgramAccrualRuleTypeV20230925 {
    /// A visit-based accrual rule. A buyer earns points for each visit. You can specify the minimum purchase required.
    Visit,
    /// A spend-based accrual rule. A buyer earns points based on the amount spent.
    Spend,
    /// An accrual rule based on an item variation. For example, accrue points for purchasing a coffee.
    ItemVariation,
    /// An accrual rule based on an item category. For example, accrue points for purchasing any item in the "hot drink" category: coffee, tea, or hot cocoa.
    Category,
}
