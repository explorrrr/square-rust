//! BusinessHours

use serde::{Deserialize, Serialize};

/// The hours of operation for a location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessHours {
    /// The list of time periods during which the business is open. There can be at most 10 periods per day.
    pub periods: Option<Vec<BusinessHoursPeriod>>,
}
