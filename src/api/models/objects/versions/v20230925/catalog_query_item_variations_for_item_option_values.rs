//! CatalogQueryItemVariationsForItemOptionValues

use serde::{Deserialize, Serialize};

/// The query filter to return the item variations containing the specified item option value IDs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogQueryItemVariationsForItemOptionValuesV20230925 {
    /// A set of CatalogItemOptionValue IDs to be used to find associated CatalogItemVariations. All ItemVariations that contain all of the given Item Option Values (in any order) will be returned.
    pub item_option_value_ids: Option<Vec<String>>,
}
