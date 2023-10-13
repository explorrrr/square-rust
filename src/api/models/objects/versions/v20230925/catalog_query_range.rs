//! CatalogQueryRange

use serde::{Deserialize, Serialize};

/// The query filter to return the search result whose named attribute values fall between the specified range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogQueryRangeV20230925 {
    /// The name of the attribute to be searched.
    /// Min Length 1
    pub attribute_name: String,
    /// The desired minimum value for the search attribute (inclusive).
    pub attribute_min_value: Option<i64>,
    /// The desired maximum value for the search attribute (inclusive).
    pub attribute_max_value: Option<i64>,
}
