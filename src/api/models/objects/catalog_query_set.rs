//! CatalogQuerySet

use serde::{Deserialize, Serialize};

/// The query filter to return the search result(s) by exact match of the specified attribute_name and any of the attribute_values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogQuerySet {
    /// The name of the attribute to be searched.
    /// Min Length 1
    pub attribute_name: String,
    /// The desired values of the search attribute.
    /// Min Length 1
    pub attribute_values: Vec<String>,
}
