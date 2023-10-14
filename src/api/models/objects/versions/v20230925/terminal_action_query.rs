//! TerminalActionQuery

use serde::{Deserialize, Serialize};

use super::{
    terminal_action_query_filter::TerminalActionQueryFilterV20230925,
    terminal_action_query_sort::TerminalActionQuerySortV20230925,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalActionQueryV20230925 {
    /// Options for filtering returned TerminalActions
    pub filter: Option<TerminalActionQueryFilterV20230925>,
    /// Option for sorting returned TerminalAction objects.
    pub sort: Option<TerminalActionQuerySortV20230925>,
}
