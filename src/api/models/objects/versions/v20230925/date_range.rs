//! DateRange

use serde::{Deserialize, Serialize};

/// A range defined by two dates.
///
/// Used for filtering a query for Connect v2 objects that have date properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRangeV20230925 {
    /// A string in YYYY-MM-DD format, such as 2017-10-31, per the ISO 8601 extended format for calendar dates. The beginning of a date range (inclusive).
    pub start_date: Option<String>,
    /// A string in YYYY-MM-DD format, such as 2017-10-31, per the ISO 8601 extended format for calendar dates. The end of a date range (inclusive).
    pub end_date: Option<String>,
}
