//! CatalogPricingType Enum

use serde::{Deserialize, Serialize};

/// Indicates whether the price of a CatalogItemVariation should be entered manually at the time of sale.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogPricingTypeV20230925 {
    /// The catalog item variation's price is fixed.
    FixedPricing,
    /// The catalog item variation's price is entered at the time of sale.
    VariablePricing,
}
