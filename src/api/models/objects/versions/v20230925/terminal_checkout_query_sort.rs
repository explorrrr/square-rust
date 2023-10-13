//! TerminalCheckoutQuerySort

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::sort_order::SortOrderV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalCheckoutQuerySortV20230925 {
    /// The order in which results are listed. Default: `DESC`.
    pub sort_order: Option<SortOrderV20230925>,
}
