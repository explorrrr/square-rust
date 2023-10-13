//! CatalogObjectReference

use serde::{Deserialize, Serialize};

/// A reference to a Catalog object at a specific version.
///
/// In general this is used as an entry point into a graph of catalog objects, where the objects exist at a specific version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogObjectReferenceV20230925 {
    /// The ID of the referenced object.
    pub object_id: Option<String>,
    /// The version of the object.
    pub catalog_version: Option<i64>,
}
