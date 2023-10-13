//! CatalogQuickAmountType

use serde::{Deserialize, Serialize};

/// Determines the type of a specific Quick Amount.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogQuickAmountType {
    /// Quick Amount is created manually by the seller.
    QuickAmountTypeManual,
    /// Quick Amount is generated automatically by machine learning algorithms.
    QuickAmountTypeAuto,
}
