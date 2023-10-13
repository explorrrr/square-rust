//! ConfirmationOptions

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmationOptions {
    /// The title text to display in the confirmation screen flow on the Terminal.
    ///
    /// Min Length 1 Max Length 250
    pub title: String,
    /// The agreement details to display in the confirmation flow on the Terminal.
    ///
    /// Min Length 1 Max Length 10000
    pub body: String,
    /// The button text to display indicating the customer agrees to the displayed terms.
    ///
    /// Min Length 1 Max Length 250
    pub agree_button_text: String,
    /// The button text to display indicating the customer does not agree to the displayed terms.
    ///
    /// Min Length 1 Max Length 250
    pub disagree_button_text: Option<String>,
    /// Read only The result of the buyer’s actions when presented with the confirmation screen.
    pub decision: Option<ConfirmationDecision>,
}
