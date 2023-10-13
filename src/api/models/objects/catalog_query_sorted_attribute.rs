//! CatalogQuerySortedAttribute

use serde::{Deserialize, Serialize};

/// The query expression to specify the key to sort search results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogQuerySortedAttribute {
    /// The attribute whose value is used as the sort key.
    /// Min Length 1
    pub attribute_name: String,
    /// The first attribute value to be returned by the query. Ascending sorts will return only objects with this value or greater, while descending sorts will return only objects with this value or less. If unset, start at the beginning (for ascending sorts) or end (for descending sorts).
    pub initial_attribute_value: Option<String>,
    /// The desired sort order, "ASC" (ascending) or "DESC" (descending).
    pub sort_order: Option<String>,
}
