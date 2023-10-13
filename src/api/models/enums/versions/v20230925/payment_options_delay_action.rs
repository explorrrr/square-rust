//! PaymentOptionsDelayAction Enum

use serde::{Deserialize, Serialize};

/// Describes the action to be applied to a delayed capture payment when the delay_duration has elapsed.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentOptionsDelayActionV20230925 {
    /// Indicates that the payment should be automatically canceled when the delay duration elapses.
    Cancel,
    /// Indicates that the payment should be automatically completed when the delay duration elapses.
    Complete,
}
