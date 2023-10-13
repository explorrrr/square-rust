//! ShiftWorkday

use serde::{Deserialize, Serialize};

/// A Shift search query filter parameter that sets a range of days that a Shift must start or end in before passing the filter condition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftWorkday {
    /// Dates for fetching the shifts.
    pub date_range: Option<DateRange>,
    /// The strategy on which the dates are applied.
    pub match_shifts_by: Option<ShiftWorkdayMatcher>,
    /// Location-specific timezones convert workdays to datetime filters. Every location included in the query must have a timezone or this field must be provided as a fallback. Format: the IANA timezone database identifier for the relevant timezone.
    pub default_timezone: Option<String>,
}
