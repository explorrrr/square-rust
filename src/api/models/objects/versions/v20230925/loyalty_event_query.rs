//! LoyaltyEventQuery

use serde::{Deserialize, Serialize};

use super::loyalty_event_filter::LoyaltyEventFilterV20230925;

/// Represents a query used to search for loyalty events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventQueryV20230925 {
    /// The query filter criteria.
    pub filter: Option<LoyaltyEventFilterV20230925>,
}
