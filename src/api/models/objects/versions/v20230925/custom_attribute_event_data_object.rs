//! CustomAttributeEventDataObject

use serde::{Deserialize, Serialize};

use super::custom_attribute::CustomAttributeV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAttributeEventDataObjectV20230925 {
    /// The custom attribute.
    pub custom_attribute: Option<CustomAttributeV20230925>,
}
