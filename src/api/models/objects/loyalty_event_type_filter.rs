//! LoyaltyEventTypeFilter

use serde::{Deserialize, Serialize};

/// Filter events by event type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventTypeFilter {
    /// The loyalty event types used to filter the result. If multiple values are specified, the endpoint uses a logical OR to combine them.
    pub types: Vec<LoyaltyEventType>,
}
