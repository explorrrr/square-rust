//! LoyaltyProgramAccrualRuleSpendData

use serde::{Deserialize, Serialize};

/// Represents the details of a loyalty program accrual rule for the `SPEND` accrual type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyProgramAccrualRuleSpendData {
    /// The amount that buyers must spend to earn points. For example, given an "Earn 1 point for every $10 spent" accrual rule, a buyer who spends $105 earns 10 points.
    pub amount_money: Money,
    /// The IDs of any CATEGORY catalog objects that are excluded from points accrual.
    /// You can use the [BatchRetrieveCatalogObjects](https://developer.squareup.com/reference/square/catalog-api/batch-retrieve-catalog-objects) endpoint to retrieve information about the excluded categories.
    pub excluded_category_ids: Option<Vec<String>>,
    /// The IDs of any ITEM_VARIATION catalog objects that are excluded from points accrual.
    /// You can use the [BatchRetrieveCatalogObjects](https://developer.squareup.com/reference/square/catalog-api/batch-retrieve-catalog-objects) endpoint to retrieve information about the excluded item variations.
    pub excluded_item_variation_ids: Option<Vec<String>>,
    /// Indicates how taxes should be treated when calculating the purchase amount used for points accrual.
    pub tax_mode: LoyaltyProgramAccrualRuleTaxMode,
}
