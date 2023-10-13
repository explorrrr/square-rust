//! CustomerSort

use serde::{Deserialize, Serialize};

/// Represents the sorting criteria in a [search query](https://developer.squareup.com/reference/square/objects/CustomerQuery) that defines how to sort customer profiles returned in [SearchCustomers](https://developer.squareup.com/reference/square/customers-api/search-customers) results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSort {
    /// Indicates the fields to use as the sort key, which is either the default set of fields or `created_at`.
    ///
    /// The default value is `DEFAULT`.
    pub field: Option<CustomerSortField>,
    /// Indicates the order in which results should be sorted based on the sort field value. Strings use standard alphabetic comparison to determine order. Strings representing numbers are sorted as strings.
    ///
    /// The default value is `ASC`.
    pub order: Option<SortOrder>,
}
