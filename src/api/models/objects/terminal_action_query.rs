//! TerminalActionQuery

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalActionQuery {
    /// Options for filtering returned TerminalActions
    pub filter: Option<TerminalActionQueryFilter>,
    /// Option for sorting returned TerminalAction objects.
    pub sort: Option<TerminalActionQuerySort>,
}
