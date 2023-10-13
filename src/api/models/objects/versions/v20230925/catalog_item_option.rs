//! CatalogItemOption

use serde::{Deserialize, Serialize};

use crate::api::models::objects::versions::v20230925::catalog_item_option_value::CatalogItemOptionValueV20230925;

/// A group of variations for a CatalogItem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogItemOptionV20230925 {
    /// The item option's display name for the seller. Must be unique across all item options. This is a searchable attribute for use in applicable query filters.
    pub name: Option<String>,
    /// The item option's display name for the customer. This is a searchable attribute for use in applicable query filters.
    pub display_name: Option<String>,
    /// The item option's human-readable description. Displayed in the Square Point of Sale app for the seller and in the Online Store or on receipts for the buyer. This is a searchable attribute for use in applicable query filters.
    pub description: Option<String>,
    /// If true, display colors for entries in values when present.
    pub show_colors: Option<bool>,
    /// A list of CatalogObjects containing the CatalogItemOptionValues for this item.
    pub values: Option<Vec<CatalogItemOptionValueV20230925>>,
}
