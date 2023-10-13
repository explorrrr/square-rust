//! AfterpayDetails

use serde::{Deserialize, Serialize};

/// Additional details about Afterpay payments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AfterpayDetailsV20230925 {
    /// Email address on the buyer's Afterpay account.
    /// Max Length: 255
    pub email_address: Option<String>,
}
