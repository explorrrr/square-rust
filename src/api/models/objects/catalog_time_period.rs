//! CatalogTimePeriod

use serde::{Deserialize, Serialize};

/// Represents a time period - either a single period or a repeating period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogTimePeriod {
    /// An iCalendar (RFC 5545) event, which specifies the name, timing, duration and recurrence of this time period.
    ///
    /// Example:
    ///
    /// ```no_run
    /// DTSTART:20190707T180000
    /// DURATION:P2H
    /// RRULE:FREQ=WEEKLY;BYDAY=MO,WE,FR
    /// ```
    ///
    /// Only SUMMARY, DTSTART, DURATION and RRULE fields are supported. DTSTART must be in local (unzoned) time format. Note that while BEGIN:VEVENT and END:VEVENT is not required in the request. The response will always include them.
    pub event: Option<String>,
}
