//! CatalogQuickAmountsSettings

use serde::{Deserialize, Serialize};

// A parent Catalog Object model represents a set of Quick Amounts and the settings control the amounts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogQuickAmountsSettings {
    /// Represents the option seller currently uses on Quick Amounts.
    pub option: String,
    /// Represents location's eligibility for auto amounts The boolean should be consistent with whether there are AUTO amounts in the amounts.
    pub eligible_for_auto_amounts: Option<bool>,
    /// Represents a set of Quick Amounts at this location.
    pub amounts: Option<Vec<CatalogQuickAmount>>,
}
