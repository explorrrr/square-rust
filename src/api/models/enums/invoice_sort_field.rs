//! InvoiceSortField Enum

use serde::{Deserialize, Serialize};

/// The field to use for sorting.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceSortField {
    /// The field works as follows:
    /// - If the invoice is a draft, it uses the invoice created_at date.
    /// - If the invoice is scheduled for publication, it uses the scheduled_at date.
    /// - If the invoice is published, it uses the invoice publication date.
    InvoiceSortDate,
}
