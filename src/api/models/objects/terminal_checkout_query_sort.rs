//! TerminalCheckoutQuerySort

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalCheckoutQuerySort {
    /// The order in which results are listed. Default: `DESC`.
    pub sort_order: Option<SortOrder>,
}
