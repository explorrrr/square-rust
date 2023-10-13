//! InvoiceCustomFieldPlacement Enum

use serde::{Deserialize, Serialize};

/// Indicates where to render a custom field on the Square-hosted invoice page and in emailed or PDF copies of the invoice.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceCustomFieldPlacementV20230925 {
    /// Render the custom field above the invoice line items.
    AboveLineItems,
    /// Render the custom field below the invoice line items.
    BelowLineItems,
}
