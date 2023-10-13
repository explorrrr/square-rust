//! CatalogCustomAttributeDefinitionNumberConfig

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogCustomAttributeDefinitionNumberConfigV20230925 {
    /// An integer between 0 and 5 that represents the maximum number of positions allowed after the decimal in number custom attribute values For example:
    /// if the precision is 0, the quantity can be 1, 2, 3, etc.
    /// if the precision is 1, the quantity can be 0.1, 0.2, etc.
    /// if the precision is 2, the quantity can be 0.01, 0.12, etc.
    /// Default: 5
    /// Max: 5
    pub precision: Option<i32>,
}
