//! CustomerQuery

use serde::{Deserialize, Serialize};

use super::{customer_filter::CustomerFilterV20230925, customer_sort::CustomerSortV20230925};

/// Represents filtering and sorting criteria for a [SearchCustomers](https://developer.squareup.com/reference/square/customers-api/search-customers) request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerQueryV20230925 {
    /// The filtering criteria for the search query. A query can contain multiple filters in any combination. Multiple filters are combined as AND statements.
    ///
    /// Note: Combining multiple filters as OR statements is not supported. Instead, send multiple single-filter searches and join the result sets.
    pub filter: Option<CustomerFilterV20230925>,
    /// Sorting criteria for query results. The default behavior is to sort customers alphabetically by given_name and family_name.
    pub sort: Option<CustomerSortV20230925>,
}
