//! CatalogObjectBatch

use serde::{Deserialize, Serialize};

use crate::api::models::objects::versions::v20230925::catalog_object::CatalogObjectV20230925;


/// A batch of catalog objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogObjectBatchV20230925 {
    /// A list of CatalogObjects belonging to this batch.
    pub objects: Vec<CatalogObjectV20230925>,
}
