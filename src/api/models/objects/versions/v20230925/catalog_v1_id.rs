//! CatalogV1Id

use serde::{Deserialize, Serialize};

/// A Square API V1 identifier of an item, including the object ID and its associated location ID.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogV1IdV20230925 {
    /// The ID for an object used in the Square API V1, if the object ID differs from the Square API V2 object ID.
    pub catalog_v1_id: Option<String>,
    /// The ID of the Location this Connect V1 ID is associated with.
    pub location_id: Option<String>,
}
