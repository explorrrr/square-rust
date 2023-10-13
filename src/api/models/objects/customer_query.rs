//! CustomerQuery

use serde::{Deserialize, Serialize};

/// Represents filtering and sorting criteria for a [SearchCustomers](https://developer.squareup.com/reference/square/customers-api/search-customers) request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerQuery {
    /// The filtering criteria for the search query. A query can contain multiple filters in any combination. Multiple filters are combined as AND statements.
    ///
    /// Note: Combining multiple filters as OR statements is not supported. Instead, send multiple single-filter searches and join the result sets.
    pub filter: Option<CustomerFilter>,
    /// Sorting criteria for query results. The default behavior is to sort customers alphabetically by given_name and family_name.
    pub sort: Option<CustomerSort>,
}
