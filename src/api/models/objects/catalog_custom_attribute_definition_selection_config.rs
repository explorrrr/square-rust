//! CatalogCustomAttributeDefinitionSelectionConfig

use serde::{Deserialize, Serialize};

/// Configuration associated with SELECTION-type custom attribute definitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogCustomAttributeDefinitionSelectionConfig {
    /// The maximum number of selections that can be set. The maximum value for this attribute is 100. The default value is 1. The value can be modified, but changing the value will not affect existing custom attribute values on objects. Clients need to handle custom attributes with more selected values than allowed by this limit.
    /// Max 100
    pub max_allowed_selections: Option<i32>,
    /// The set of valid CatalogCustomAttributeSelections. Up to a maximum of 100 selections can be defined. Can be modified.
    pub allowed_selections: Option<Vec<CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection>>,
}
