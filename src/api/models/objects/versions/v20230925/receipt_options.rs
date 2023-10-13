//! ReceiptOptions

use serde::{Deserialize, Serialize};

/// Describes receipt action fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptOptionsV20230925 {
    /// The reference to the Square payment ID for the receipt.
    pub payment_id: String,
    /// Instructs the device to print the receipt without displaying the receipt selection screen. Requires printer_enabled set to true. Defaults to false.
    pub print_only: Option<bool>,
    /// Identify the receipt as a reprint rather than an original receipt. Defaults to false.
    pub is_duplicate: Option<bool>,
}
