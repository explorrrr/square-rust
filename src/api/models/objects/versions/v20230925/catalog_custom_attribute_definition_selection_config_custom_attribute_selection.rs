//! CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelection

use serde::{Deserialize, Serialize};

/// A named selection for this SELECTION-type custom attribute definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogCustomAttributeDefinitionSelectionConfigCustomAttributeSelectionV20230925 {
    /// Unique ID set by Square.
    pub uid: Option<String>,
    /// Selection name, unique within allowed_selections.
    /// Min Length 1
    /// Max Length 255
    pub name: String,
}
