//! InvoiceQuery

use serde::{Deserialize, Serialize};

use super::{invoice_filter::InvoiceFilterV20230925, invoice_sort::InvoiceSortV20230925};

/// Describes query criteria for searching invoices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceQueryV20230925 {
    /// Query filters to apply in searching invoices. For more information, see [Search for invoices](https://developer.squareup.com/docs/invoices-api/retrieve-list-search-invoices#search-invoices).
    pub filter: InvoiceFilterV20230925,
    /// Describes the sort order for the search result.
    pub sort: Option<InvoiceSortV20230925>,
}
