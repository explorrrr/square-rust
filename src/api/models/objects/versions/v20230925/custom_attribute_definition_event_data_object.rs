//! CustomAttributeDefinitionEventDataObject

use serde::{Deserialize, Serialize};

use crate::api::models::objects::versions::v20230925::custom_attribute_definition::CustomAttributeDefinitionV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAttributeDefinitionEventDataObjectV20230925 {
    /// The custom attribute definition.
    pub custom_attribute_definition: Option<CustomAttributeDefinitionV20230925>,
}
