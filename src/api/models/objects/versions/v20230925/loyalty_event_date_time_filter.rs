//! LoyaltyEventDateTimeFilter

use serde::{Deserialize, Serialize};

use super::time_range::TimeRangeV20230925;

/// Filter events by date time range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventDateTimeFilterV20230925 {
    /// The created_at date time range used to filter the result.
    pub created_at: TimeRangeV20230925,
}
