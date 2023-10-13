//! SelectOptions

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOptions {
    /// The title text to display in the select flow on the Terminal.
    ///
    /// Min Length 1
    /// Max Length 250
    pub title: String,
    /// The body text to display in the select flow on the Terminal.
    ///
    /// Min Length 1
    /// Max Length 10000
    pub body: String,
    /// Represents the buttons/options that should be displayed in the select flow on the Terminal.
    pub options: Vec<SelectOption>,
    /// Read only The buyer’s selected option.
    pub selected_option: Option<SelectOption>,
}
