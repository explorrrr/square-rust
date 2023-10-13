//! RefundStatus Enum

use serde::{Deserialize, Serialize};

/// Indicates a refund's current status.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefundStatusV20230925 {
    /// The refund is pending.
    Pending,
    /// The refund has been approved by Square.
    Approved,
    /// The refund has been rejected by Square.
    Rejected,
    /// The refund failed.
    Failed,
}
