//! InvoiceQuery

use serde::{Deserialize, Serialize};

/// Describes query criteria for searching invoices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceQuery {
    /// Query filters to apply in searching invoices. For more information, see [Search for invoices](https://developer.squareup.com/docs/invoices-api/retrieve-list-search-invoices#search-invoices).
    pub filter: InvoiceFilter,
    /// Describes the sort order for the search result.
    pub sort: Option<InvoiceSort>,
}
