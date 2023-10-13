//! WorkweekConfig

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::weekday::WeekdayV20230925;

/// Sets the day of the week and hour of the day that a business starts a workweek.
///
/// This is used to calculate overtime pay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkweekConfigV20230925 {
    /// The UUID for this object.
    pub id: Option<String>,
    /// The day of the week on which a business week starts for compensation purposes.
    pub start_of_week: WeekdayV20230925,
    /// The local time at which a business week starts. Represented as a string in HH:MM format (HH:MM:SS is also accepted, but seconds are truncated).
    ///
    /// Min Length
    /// 1
    pub start_of_day_local_time: WeekdayV20230925,
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
    /// Read only A read-only timestamp in RFC 3339 format; presented in UTC.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
}
