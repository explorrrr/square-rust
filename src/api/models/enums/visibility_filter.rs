//! VisibilityFilter Enum

use serde::{Deserialize, Serialize};

/// Enumeration of visibility-filter values used to set the ability to view custom attributes or custom attribute definitions.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VisibilityFilter {
    /// All custom attributes or custom attribute definitions.
    All,
    /// All custom attributes or custom attribute definitions with the visibility field set to VISIBILITY_READ_ONLY or VISIBILITY_READ_WRITE_VALUES.
    Read,
    /// All custom attributes or custom attribute definitions with the visibility field set to VISIBILITY_READ_WRITE_VALUES.
    ReadWrite,
}
