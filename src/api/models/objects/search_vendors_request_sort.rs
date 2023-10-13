//! SearchVendorsRequestSort

use serde::{Deserialize, Serialize};

/// Defines a sorter used to sort results from [SearchVendors](https://developer.squareup.com/reference/square/vendors-api/search-vendors).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchVendorsRequestSort {
    /// Specifies the sort key to sort the returned vendors.
    pub field: Option<SearchVendorsRequestSortField>,
    /// Specifies the sort order for the returned vendors.
    pub order: Option<SortOrder>,
}
