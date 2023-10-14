//! JobAssignment

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;
use crate::api::models::enums::versions::v20230925::job_assignment_pay_type::JobAssignmentPayTypeV20230925;

/// An object describing a job that a team member is assigned to.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobAssignmentV20230925 {
    /// The title of the job.
    ///
    /// Min Length 1
    pub job_title: String,
    /// The current pay type for the job assignment used to calculate the pay amount in a pay period.
    pub pay_type: JobAssignmentPayTypeV20230925,
    /// The hourly pay rate of the job.
    pub hourly_rate: Option<MoneyV20230925>,
    /// The total pay amount for a 12-month period on the job. Set if the job PayType is SALARY.
    pub annual_rate: Option<MoneyV20230925>,
    /// The planned hours per week for the job. Set if the job PayType is SALARY.
    pub weekly_hours: Option<i32>,
}
