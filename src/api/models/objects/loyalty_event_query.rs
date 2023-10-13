//! LoyaltyEventQuery

use serde::{Deserialize, Serialize};

/// Represents a query used to search for loyalty events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventQuery {
    /// The query filter criteria.
    pub filter: Option<LoyaltyEventFilter>,
}
