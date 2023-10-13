//! TerminalActionQuerySort

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalActionQuerySort {
    /// The order in which results are listed.
    /// - `ASC` - Oldest to newest.
    /// - `DESC` - Newest to oldest (default).
    pub sort_order: Option<SortOrder>,
}
