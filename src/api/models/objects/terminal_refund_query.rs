//! TerminalRefundQuery

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalRefundQuery {
    /// The filter to return results by `TerminalRefund` `status`.
    pub filter: Option<TerminalRefundQueryFilter>,
    /// The order to sort the results.
    pub sort: Option<TerminalRefundQuerySort>,
}
