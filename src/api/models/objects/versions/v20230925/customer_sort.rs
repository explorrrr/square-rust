//! CustomerSort

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::{customer_sort_filed::CustomerSortFieldV20230925, sort_order::SortOrderV20230925};

/// Represents the sorting criteria in a [search query](https://developer.squareup.com/reference/square/objects/CustomerQuery) that defines how to sort customer profiles returned in [SearchCustomers](https://developer.squareup.com/reference/square/customers-api/search-customers) results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSortV20230925 {
    /// Indicates the fields to use as the sort key, which is either the default set of fields or `created_at`.
    ///
    /// The default value is `DEFAULT`.
    pub field: Option<CustomerSortFieldV20230925>,
    /// Indicates the order in which results should be sorted based on the sort field value. Strings use standard alphabetic comparison to determine order. Strings representing numbers are sorted as strings.
    ///
    /// The default value is `ASC`.
    pub order: Option<SortOrderV20230925>,
}
