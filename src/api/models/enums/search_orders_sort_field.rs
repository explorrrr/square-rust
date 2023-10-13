//! SearchOrdersSortField Enum

use serde::{Deserialize, Serialize};

/// Specifies which timestamp to use to sort SearchOrder results.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SearchOrdersSortField {
    /// The time when the order was created, in RFC-3339 format. If you are also filtering for a time range in this query, you must set the CREATED_AT field in your DateTimeFilter.
    CreatedAt,
    /// The time when the order last updated, in RFC-3339 format. If you are also filtering for a time range in this query, you must set the UPDATED_AT field in your DateTimeFilter.
    UpdatedAt,
    /// The time when the order was closed, in RFC-3339 format. If you use this value, you must also set a StateFilter with closed states. If you are also filtering for a time range in this query, you must set the CLOSED_AT field in your DateTimeFilter.
    ClosedAt,
}
