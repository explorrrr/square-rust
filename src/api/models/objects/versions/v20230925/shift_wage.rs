//! ShiftWage

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;

/// The hourly wage rate used to compensate an employee for this shift.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftWageV20230925 {
    /// The name of the job performed during this shift.
    pub title: Option<String>,
    /// Can be a custom-set hourly wage or the calculated effective hourly wage based on the annual wage and hours worked per week.
    pub hourly_rate: Option<MoneyV20230925>,
    /// Read only The id of the job performed during this shift. Square labor-reporting UIs might group shifts together by id. This cannot be used to retrieve the job.
    pub job_id: Option<String>,
}
