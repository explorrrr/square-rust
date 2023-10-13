//! InvoiceSort

use serde::{Deserialize, Serialize};

/// Identifies the sort field and sort order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceSort {
    /// The field to use for sorting.
    pub field: InvoiceSortField,
    /// The order to use for sorting the results.
    pub order: Option<SortOrder>,
}
