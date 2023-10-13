//! WageSetting

use serde::{Deserialize, Serialize};

use super::job_assignment::JobAssignmentV20230925;

/// An object representing a team member's wage information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WageSettingV20230925 {
    /// The unique ID of the TeamMember whom this wage setting describes.
    pub team_member_id: Option<String>,
    /// Required. The ordered list of jobs that the team member is assigned to. The first job assignment is considered the team member's primary job.
    ///
    /// The minimum length is 1 and the maximum length is 12.
    pub job_assignments: Option<Vec<JobAssignmentV20230925>>,
    /// Whether the team member is exempt from the overtime rules of the seller's country.
    pub is_overtime_exempt: Option<bool>,
    /// Used for resolving concurrency issues. The request fails if the version provided does not match the server version at the time of the request. If not provided, Square executes a blind write, potentially overwriting data from another write. For more information, see [optimistic concurrency](https://developer.squareup.com/docs/working-with-apis/optimistic-concurrency).
    pub version: Option<i32>,
    /// Read only The timestamp, in RFC 3339 format, describing when the wage setting object was created. For example, `"2018-10-04T04:00:00-07:00"` or `"2019-02-05T12:00:00Z"`.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: `2020-01-26T02:25:34Z`
    ///
    /// Pacific Standard Time with UTC offset: `2020-01-25T18:25:34-08:00`
    pub created_at: Option<String>,
    /// Read only The timestamp, in RFC 3339 format, describing when the wage setting object was last updated. For example, `"2018-10-04T04:00:00-07:00"` or `"2019-02-05T12:00:00Z"`.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: `2020-01-26T02:25:34Z`
    ///
    /// Pacific Standard Time with UTC offset: `2020-01-25T18:25:34-08:00`
    pub updated_at: Option<String>,
}
