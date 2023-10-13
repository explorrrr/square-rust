//! TerminalCheckoutQuery

use serde::{Deserialize, Serialize};

use super::{terminal_checkout_query_filter::TerminalCheckoutQueryFilterV20230925, terminal_checkout_query_sort::TerminalCheckoutQuerySortV20230925};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalCheckoutQueryV20230925 {
    /// Options for filtering returned TerminalCheckout objects.
    pub filter: Option<TerminalCheckoutQueryFilterV20230925>,
    /// Option for sorting returned TerminalCheckout objects.
    pub sort: Option<TerminalCheckoutQuerySortV20230925>,
}
