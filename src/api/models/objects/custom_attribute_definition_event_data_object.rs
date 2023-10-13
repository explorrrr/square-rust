//! CustomAttributeDefinitionEventDataObject

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAttributeDefinitionEventDataObject {
    /// The custom attribute definition.
    pub custom_attribute_definition: Option<CustomAttributeDefinition>,
}
