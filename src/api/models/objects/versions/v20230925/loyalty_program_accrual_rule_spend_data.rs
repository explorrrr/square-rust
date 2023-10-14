//! LoyaltyProgramAccrualRuleSpendData

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;
use crate::api::models::enums::versions::v20230925::loyalty_program_accrual_rule_tax_mode::LoyaltyProgramAccrualRuleTaxModeV20230925;

/// Represents the details of a loyalty program accrual rule for the `SPEND` accrual type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyProgramAccrualRuleSpendDataV20230925 {
    /// The amount that buyers must spend to earn points. For example, given an "Earn 1 point for every $10 spent" accrual rule, a buyer who spends $105 earns 10 points.
    pub amount_money: MoneyV20230925,
    /// The IDs of any CATEGORY catalog objects that are excluded from points accrual.
    /// You can use the [BatchRetrieveCatalogObjects](https://developer.squareup.com/reference/square/catalog-api/batch-retrieve-catalog-objects) endpoint to retrieve information about the excluded categories.
    pub excluded_category_ids: Option<Vec<String>>,
    /// The IDs of any ITEM_VARIATION catalog objects that are excluded from points accrual.
    /// You can use the [BatchRetrieveCatalogObjects](https://developer.squareup.com/reference/square/catalog-api/batch-retrieve-catalog-objects) endpoint to retrieve information about the excluded item variations.
    pub excluded_item_variation_ids: Option<Vec<String>>,
    /// Indicates how taxes should be treated when calculating the purchase amount used for points accrual.
    pub tax_mode: LoyaltyProgramAccrualRuleTaxModeV20230925,
}
