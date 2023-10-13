//! CatalogQuickAmountsSettings

use serde::{Deserialize, Serialize};

use crate::api::models::objects::versions::v20230925::catalog_quick_amount::CatalogQuickAmountV20230925;

// A parent Catalog Object model represents a set of Quick Amounts and the settings control the amounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogQuickAmountsSettingsV20230925 {
    /// Represents the option seller currently uses on Quick Amounts.
    pub option: String,
    /// Represents location's eligibility for auto amounts The boolean should be consistent with whether there are AUTO amounts in the amounts.
    pub eligible_for_auto_amounts: Option<bool>,
    /// Represents a set of Quick Amounts at this location.
    pub amounts: Option<Vec<CatalogQuickAmountV20230925>>,
}
