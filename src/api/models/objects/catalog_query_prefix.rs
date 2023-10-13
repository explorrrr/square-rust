//! CatalogQueryPrefix

use serde::{Deserialize, Serialize};

/// The query filter to return the search result whose named attribute values are prefixed by the specified attribute value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogQueryPrefix {
    /// The name of the attribute to be searched.
    /// Min Length 1
    pub attribute_name: String,
    /// The desired prefix of the search attribute value.
    /// Min Length 1
    pub attribute_prefix: String,
}
