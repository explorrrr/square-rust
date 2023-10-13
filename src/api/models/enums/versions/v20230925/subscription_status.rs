//! SubscriptionStatus Enum

use serde::{Deserialize, Serialize};

/// Supported subscription statuses.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionStatusV20230925 {
    /// The subscription is pending to start in the future.
    Pending,
    /// The subscription is active.
    Active,
    /// The subscription is canceled.
    Canceled,
    /// The subscription is deactivated.
    Deactivated,
    /// The subscription is paused.
    Paused,
}
