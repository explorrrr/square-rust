//! CatalogCustomAttributeDefinitionAppVisibility enum

use serde::{Deserialize, Serialize};

/// Defines the visibility of a custom attribute to applications other than their creating application.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogCustomAttributeDefinitionAppVisibilityV20230925 {
    /// Other applications cannot read this custom attribute.
    AppVisibilityHidden,
    /// Other applications can read this custom attribute definition and values.
    AppVisibilityReadOnly,
    /// Other applications can read and write custom attribute values on objects. They can read but cannot edit the custom attribute definition.
    AppVisibilityReadWriteValues,
}
