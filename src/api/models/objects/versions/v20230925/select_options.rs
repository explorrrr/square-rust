//! SelectOptions

use serde::{Deserialize, Serialize};

use super::select_option::SelectOptionV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOptionsV20230925 {
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
    pub options: Vec<SelectOptionV20230925>,
    /// Read only The buyer’s selected option.
    pub selected_option: Option<SelectOptionV20230925>,
}
