//! SearchOrdersQuery

use serde::{Deserialize, Serialize};

use super::{search_orders_filter::SearchOrdersFilterV20230925, search_orders_sort::SearchOrdersSortV20230925};

/// Contains query criteria for the search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOrdersQueryV20230925 {
    /// Criteria to filter results by.
    pub filter: Option<SearchOrdersFilterV20230925>,
    /// Criteria to sort results by.
    pub sort: Option<SearchOrdersSortV20230925>,
}
