//! LoyaltyEventDatetimeFilter

use serde::{Deserialize, Serialize};

/// Filter events by date time range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventDatetimeFilter {
    /// The created_at date time range used to filter the result.
    pub created_at: TimeRange,
}
