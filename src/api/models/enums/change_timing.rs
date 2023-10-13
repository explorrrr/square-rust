//! ChangeTiming Enum

use serde::{Deserialize, Serialize};

/// Supported timings when a pending change, as an action, takes place to a subscription.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChangeTiming {
    /// The action occurs immediately.
    Immediate,
    /// The action occurs at the end of the billing cycle.
    EndOfBillingCycle,
}
