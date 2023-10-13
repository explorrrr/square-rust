//! LoyaltyProgramAccrualRuleCategoryData

use serde::{Deserialize, Serialize};

/// Represents additional data for rules with the CATEGORY accrual type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyProgramAccrualRuleCategoryData {
    /// The ID of the CATEGORY [catalog object](https://developer.squareup.com/reference/square/objects/CatalogObject) that buyers can purchase to earn points.
    ///
    /// Min Length 1
    pub category_id: String,
}
