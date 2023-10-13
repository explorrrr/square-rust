//! CatalogQuickAmountsSettingsOption Enum

use serde::{Deserialize, Serialize};

/// Determines a seller's option on Quick Amounts feature.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogQuickAmountsSettingsOptionV20230925 {
    /// Option for seller to disable Quick Amounts.
    Disabled,
    /// Option for seller to choose manually created Quick Amounts.
    Manual,
    /// Option for seller to choose automatically created Quick Amounts.
    Auto,
}
