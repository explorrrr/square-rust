//! TeamMemberWage

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;

/// The hourly wage rate that a team member earns on a Shift for doing the job specified by the title property of this object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMemberWageV20230925 {
    /// The UUID for this object.
    pub id: Option<String>,
    /// The TeamMember that this wage is assigned to.
    pub team_member_id: Option<String>,
    /// The job title that this wage relates to.
    pub title: Option<String>,
    /// Can be a custom-set hourly wage or the calculated effective hourly wage based on the annual wage and hours worked per week.
    pub hourly_rate: Option<MoneyV20230925>,
    /// An identifier for the job that this wage relates to. This cannot be used to retrieve the job.
    pub job_id: Option<String>,
}
