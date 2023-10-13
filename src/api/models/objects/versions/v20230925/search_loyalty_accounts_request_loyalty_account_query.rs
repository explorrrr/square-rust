//! SearchLoyaltyAccountsRequestLoyaltyAccountQuery

use serde::{Deserialize, Serialize};

use super::loyalty_account_mapping::LoyaltyAccountMappingV20230925;

/// The search criteria for the loyalty accounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchLoyaltyAccountsRequestLoyaltyAccountQueryV20230925 {
    /// The set of mappings to use in the loyalty account search.
    ///
    /// This cannot be combined with customer_ids.
    ///
    /// Max: 30 mappings
    pub mappings: Option<Vec<LoyaltyAccountMappingV20230925>>,
    /// The set of customer IDs to use in the loyalty account search.
    ///
    /// This cannot be combined with mappings.
    ///
    /// Max: 30 customer IDs
    pub customer_ids: Option<Vec<String>>,
}
