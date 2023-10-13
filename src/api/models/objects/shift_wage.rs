//! ShiftWage

use serde::{Deserialize, Serialize};

/// The hourly wage rate used to compensate an employee for this shift.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftWage {
    /// The name of the job performed during this shift.
    pub title: Option<String>,
    /// Can be a custom-set hourly wage or the calculated effective hourly wage based on the annual wage and hours worked per week.
    pub hourly_rate: Option<Money>,
    /// Read only The id of the job performed during this shift. Square labor-reporting UIs might group shifts together by id. This cannot be used to retrieve the job.
    pub job_id: Option<String>,
}
