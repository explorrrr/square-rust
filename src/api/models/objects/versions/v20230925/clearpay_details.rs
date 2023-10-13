//! ClearpayDetails

use serde::{Deserialize, Serialize};

/// Additional details about Clearpay payments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearpayDetailsV20230925 {
    /// Email address on the buyer's Clearpay account.
    /// Max Length 255
    pub email_address: Option<String>,
}