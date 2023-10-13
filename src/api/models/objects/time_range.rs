//! TimeRange

use serde::{Deserialize, Serialize};

/// Represents a generic time range.
///
/// The start and end values are represented in RFC 3339 format. Time ranges are customized to be inclusive or exclusive based on the needs of a particular endpoint. Refer to the relevant endpoint-specific documentation to determine how time ranges are handled.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    /// A datetime value in RFC 3339 format indicating when the time range starts.
    pub start_at: Option<String>,
    /// A datetime value in RFC 3339 format indicating when the time range ends.
    pub end_at: Option<String>,
}
