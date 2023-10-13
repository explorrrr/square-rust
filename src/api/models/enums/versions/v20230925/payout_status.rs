//! PayoutStatus Enum

use serde::{Deserialize, Serialize};

/// Payout status types
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayoutStatusV20230925 {
    /// Indicates that the payout was successfully sent to the banking destination.
    Sent,
    /// Indicates that the payout was rejected by the banking destination.
    Failed,
    /// Indicates that the payout has successfully completed.
    Paid,
}
