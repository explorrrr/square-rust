//! TerminalRefundQuery

use serde::{Deserialize, Serialize};

use super::{terminal_refund_query_filter::TerminalRefundQueryFilterV20230925, terminal_refund_query_sort::TerminalRefundQuerySortV20230925};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalRefundQueryV20230925 {
    /// The filter to return results by `TerminalRefund` `status`.
    pub filter: Option<TerminalRefundQueryFilterV20230925>,
    /// The order to sort the results.
    pub sort: Option<TerminalRefundQuerySortV20230925>,
}
