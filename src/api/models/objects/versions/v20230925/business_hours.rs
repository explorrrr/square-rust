//! BusinessHours

use serde::{Deserialize, Serialize};

use crate::api::models::objects::versions::v20230925::business_hours_period::BusinessHoursPeriodV20230925;

/// The hours of operation for a location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessHoursV20230925 {
    /// The list of time periods during which the business is open. There can be at most 10 periods per day.
    pub periods: Option<Vec<BusinessHoursPeriodV20230925>>,
}
