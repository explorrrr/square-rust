//! SubscriptionEventInfo

use serde::{Deserialize, Serialize};

/// Provides information about the subscription event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionEventInfo {
    /// A human-readable explanation for the event.
    pub detail: Option<String>,
    /// An info code indicating the subscription event that occurred.
    pub code: Option<SubscriptionEventInfoCode>,
}
