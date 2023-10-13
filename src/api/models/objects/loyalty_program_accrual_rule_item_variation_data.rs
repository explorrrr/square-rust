//! LoyaltyProgramAccrualRuleItemVariationData

use serde::{Deserialize, Serialize};

/// Represents additional data for rules with the ITEM_VARIATION accrual type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyProgramAccrualRuleItemVariationData {
    /// The ID of the ITEM_VARIATION [catalog object](https://developer.squareup.com/reference/square/objects/CatalogObject) that buyers can purchase to earn points.
    ///
    /// Min Length 1
    pub item_variation_id: String,
}
