//! CustomAttributeEventDataObject

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAttributeEventDataObject {
    /// The custom attribute.
    pub custom_attribute: Option<CustomAttribute>,
}
