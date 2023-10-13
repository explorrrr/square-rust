//! TenderBankAccountDetailsStatus Enum

use serde::{Deserialize, Serialize};

/// Indicates the bank account payment's current status.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TenderBankAccountDetailsStatusV20230925 {
    /// The bank account payment is in progress.
    Pending,
    /// The bank account payment has been completed.
    Completed,
    /// The bank account payment failed.
    Failed,
}
