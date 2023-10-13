//! SelectOption

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
    /// The reference id for the option.
    ///
    /// Min Length 1
    /// Max Length 40
    pub reference_id: String,
    /// The title text that displays in the select option button.
    ///
    /// Min Length 1
    /// Max Length 250
    pub title: String,
}
