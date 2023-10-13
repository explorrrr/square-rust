//! Shift

use serde::{Deserialize, Serialize};

/// A record of the hourly rate, start, and end times for a single work shift for an employee.
///
/// This might include a record of the start and end times for breaks taken during the shift.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shift {
    /// The UUID for this object.
    ///
    /// Max Length 255
    pub id: Option<String>,
    /// The ID of the location this shift occurred at. The location should be based on where the employee clocked in.
    pub location_id: Option<String>,
    /// The read-only convenience value that is calculated from the location based on the location_id. Format: the IANA timezone database identifier for the location timezone.
    pub timezone: Option<String>,
    /// RFC 3339; shifted to the location timezone + offset. Precision up to the minute is respected; seconds are truncated.
    pub start_at: String,
    /// RFC 3339; shifted to the timezone + offset. Precision up to the minute is respected; seconds are truncated.
    pub end_at: Option<String>,
    /// Job and pay related information. If the wage is not set on create, it defaults to a wage of zero. If the title is not set on create, it defaults to the name of the role the employee is assigned to, if any.
    pub wage: Option<ShiftWage>,
    /// A list of all the paid or unpaid breaks that were taken during this shift.
    pub breaks: Option<Vec<SquareBreak>>,
    /// Describes the working state of the current Shift.
    pub status: Option<ShiftStatus>,
    /// Used for resolving concurrency issues. The request fails if the version provided does not match the server version at the time of the request. If not provided, Square executes a blind write; potentially overwriting data from another write.
    pub version: Option<i32>,
    /// Read only A read-only timestamp in RFC 3339 format; presented in UTC.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// Read only A read-only timestamp in RFC 3339 format presented in UTC.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
    /// The ID of the team member this shift belongs to. Replaced employee_id at version "2020-08-26".
    pub team_member_id: Option<String>,
}
