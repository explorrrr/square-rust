//! CatalogIdMapping

use serde::{Deserialize, Serialize};

/// A mapping between a temporary client-supplied ID and a permanent server-generated ID.
///
/// When calling [UpsertCatalogObject](https://developer.squareup.com/reference/square/catalog-api/upsert-catalog-object) or
/// [BatchUpsertCatalogObjects](https://developer.squareup.com/reference/square/catalog-api/batch-upsert-catalog-objects) to
/// create a [CatalogObject](https://developer.squareup.com/reference/square/objects/CatalogObject) instance, you can supply a temporary ID for the to-be-created object,
/// especially when the object is to be referenced elsewhere in the same request body. This temporary ID can be any string unique within the call, but must be prefixed by "#".
///
/// After the request is submitted and the object created, a permanent server-generated ID is assigned to the new object. The permanent ID is unique across the Square catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogIdMappingV20230925 {
    /// The client-supplied temporary #-prefixed ID for a new CatalogObject.
    pub client_object_id: Option<String>,
    /// The permanent ID for the CatalogObject created by the server.
    pub object_id: Option<String>,
}
