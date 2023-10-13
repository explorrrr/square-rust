//! CatalogQueryItemsForModifierList

use serde::{Deserialize, Serialize};

/// The query filter to return the items containing the specified modifier list IDs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogQueryItemsForModifierListV20230925 {
    /// A set of CatalogModifierList IDs to be used to find associated CatalogItems.
    pub modifier_list_ids: Vec<String>,
}
