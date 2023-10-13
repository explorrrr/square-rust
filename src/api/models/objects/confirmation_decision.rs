//! ConfirmationDecision

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmationDecision {
    /// Read only The buyer's decision to the displayed terms.
    pub has_agreed: Option<bool>,
}
