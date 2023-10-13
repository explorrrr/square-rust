//! CatalogQueryItemsForTax

use serde::{Deserialize, Serialize};

/// The query filter to return the items containing the specified tax IDs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogQueryItemsForTaxV20230925 {
    /// A set of CatalogTax IDs to be used to find associated CatalogItems.
    pub tax_ids: Vec<String>,
}
