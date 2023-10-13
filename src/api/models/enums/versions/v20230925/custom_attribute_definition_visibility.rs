//! CustomAttributeDefinitionVisibility Enum

use serde::{Deserialize, Serialize};

/// The level of permission that a seller or other applications requires to view this custom attribute definition.
/// The Visibility field controls who can read and write the custom attribute values and custom attribute definition.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomAttributeDefinitionVisibilityV20230925 {
    /// The custom attribute definition and values are hidden from the seller (except on export of all seller data) and other developers.
    VisibilityHidden,
    /// The seller and other developers can read the custom attribute definition and values on resources.
    VisibilityReadOnly,
    /// The seller and other developers can read the custom attribute definition, and can read and write values on resources. A custom attribute definition can only be edited or deleted by the application that created it.
    VisibilityReadWriteValues,
}
