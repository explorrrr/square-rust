//! CatalogObjectBatch

use serde::{Deserialize, Serialize};

/// A batch of catalog objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogObjectBatch {
    /// A list of CatalogObjects belonging to this batch.
    pub objects: Vec<CatalogObject>,
}
