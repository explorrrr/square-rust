//! SearchCatalogItemsRequestStockLevel Enum

use serde::{Deserialize, Serialize};

/// Defines supported stock levels of the item inventory.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SearchCatalogItemsRequestStockLevelV20230925 {
    /// The item inventory is empty.
    Out,
    /// The item inventory is low.
    Low,
}
