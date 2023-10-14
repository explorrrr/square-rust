//! InvoiceSort

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::{
    invoice_sort_field::InvoiceSortFieldV20230925, sort_order::SortOrderV20230925,
};

/// Identifies the sort field and sort order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceSortV20230925 {
    /// The field to use for sorting.
    pub field: InvoiceSortFieldV20230925,
    /// The order to use for sorting the results.
    pub order: Option<SortOrderV20230925>,
}
