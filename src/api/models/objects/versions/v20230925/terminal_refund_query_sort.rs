//! TerminalRefundQuerySort

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalRefundQuerySortV20230925 {
    /// The order in which results are listed.
    pub sort_order: Option<String>,
}
