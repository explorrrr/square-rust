//! StandardUnitDescriptionGroup

use serde::{Deserialize, Serialize};

/// Group of standard measurement units.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardUnitDescriptionGroup {
    /// List of standard (non-custom) measurement units in this description group.
    pub standard_unit_descriptions: Option<Vec<StandardUnitDescription>>,
    /// IETF language tag.
    pub language_code: Option<String>,
}
