//! SearchOrdersQuery

use serde::{Deserialize, Serialize};

/// Contains query criteria for the search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOrdersQuery {
    /// Criteria to filter results by.
    pub filter: Option<SearchOrdersFilter>,
    /// Criteria to sort results by.
    pub sort: Option<SearchOrdersSort>,
}
