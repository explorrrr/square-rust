//! StandardUnitDescriptionGroup

use serde::{Deserialize, Serialize};

use super::standard_unit_description::StandardUnitDescriptionV20230925;

/// Group of standard measurement units.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardUnitDescriptionGroupV20230925 {
    /// List of standard (non-custom) measurement units in this description group.
    pub standard_unit_descriptions: Option<Vec<StandardUnitDescriptionV20230925>>,
    /// IETF language tag.
    pub language_code: Option<String>,
}
