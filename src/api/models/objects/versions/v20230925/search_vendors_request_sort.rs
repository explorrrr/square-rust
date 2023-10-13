//! SearchVendorsRequestSort

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::{search_vendors_request_sort_field::SearchVendorsRequestSortFieldV20230925, sort_order::SortOrderV20230925};

/// Defines a sorter used to sort results from [SearchVendors](https://developer.squareup.com/reference/square/vendors-api/search-vendors).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchVendorsRequestSortV20230925 {
    /// Specifies the sort key to sort the returned vendors.
    pub field: Option<SearchVendorsRequestSortFieldV20230925>,
    /// Specifies the sort order for the returned vendors.
    pub order: Option<SortOrderV20230925>,
}
