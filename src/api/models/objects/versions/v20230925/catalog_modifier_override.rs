//! CatalogModifierOverride

use serde::{Deserialize, Serialize};

/// Options to control how to override the default behavior of the specified modifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogModifierOverrideV20230925 {
    /// The ID of the CatalogModifier whose default behavior is being overridden.
    /// Min Length 1
    pub modifier_id: String,
    /// If true, this CatalogModifier should be selected by default for this CatalogItem.
    pub on_by_default: Option<bool>,
}
