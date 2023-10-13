//! ShiftSort

use serde::{Deserialize, Serialize};

/// Sets the sort order of search results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftSort {
    /// The field to sort on.
    pub field: Option<ShiftSortField>,
    /// The order in which results are returned. Defaults to DESC.
    pub order: Option<SortOrder>,
}
