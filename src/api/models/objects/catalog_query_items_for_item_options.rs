//! CatalogQueryItemsForItemOptions

use serde::{Deserialize, Serialize};

/// The query filter to return the items containing the specified item option IDs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogQueryItemsForItemOptions {
    /// A set of CatalogItemOption IDs to be used to find associated CatalogItems. All Items that contain all of the given Item Options (in any order) will be returned.
    pub item_option_ids: Option<Vec<String>>,
}
