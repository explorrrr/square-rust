//! TerminalActionQuerySort

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::sort_order::SortOrderV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalActionQuerySortV20230925 {
    /// The order in which results are listed.
    /// - `ASC` - Oldest to newest.
    /// - `DESC` - Newest to oldest (default).
    pub sort_order: Option<SortOrderV20230925>,
}
