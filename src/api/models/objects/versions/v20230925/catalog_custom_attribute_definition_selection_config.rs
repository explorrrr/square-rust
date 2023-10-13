//! CatalogCustomAttributeDefinitionSelectionConfig

use serde::{Deserialize, Serialize};

use crate::api::models::objects::versions::v20230925::catalog_custom_attribute_definition_selection_config_custom_attribute_selection::CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelectionV20230925;

/// Configuration associated with SELECTION-type custom attribute definitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogCustomAttributeDefinitionSelectionConfigV20230925 {
    /// The maximum number of selections that can be set. The maximum value for this attribute is 100. The default value is 1. The value can be modified, but changing the value will not affect existing custom attribute values on objects. Clients need to handle custom attributes with more selected values than allowed by this limit.
    /// Max 100
    pub max_allowed_selections: Option<i32>,
    /// The set of valid CatalogCustomAttributeSelections. Up to a maximum of 100 selections can be defined. Can be modified.
    pub allowed_selections: Option<Vec<CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelectionV20230925>>,
}
