//! TerminalRefundQuerySort

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalRefundQuerySort {
    /// The order in which results are listed.
    pub sort_order: Option<String>,
}
