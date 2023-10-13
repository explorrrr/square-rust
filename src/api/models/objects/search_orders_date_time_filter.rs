//! SearchOrdersDateTimeFilter

use serde::{Deserialize, Serialize};

/// Filter for Order objects based on whether their CREATED_AT, CLOSED_AT, or UPDATED_AT timestamps fall within a specified time range.
///
/// You can specify the time range and which timestamp to filter for. You can filter for only one time range at a time.
///
/// For each time range, the start time and end time are inclusive. If the end time is absent, it defaults to the time of the first request for the cursor.
///
/// Important: If you use the DateTimeFilter in a SearchOrders query, you must set the sort_field in OrdersSort to the same field you filter for. For example, if you set the CLOSED_AT field in DateTimeFilter, you must set the sort_field in SearchOrdersSort to CLOSED_AT. Otherwise, SearchOrders throws an error. [Learn more about filtering orders by time range](https://developer.squareup.com/docs/orders-api/manage-orders/search-orders#important-note-about-filtering-orders-by-time-range).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOrdersDateTimeFilter {
    /// Time range for filtering on the `created_at` timestamp.
    pub created_at: Option<TimeRange>,
    /// Time range for filtering on the `updated_at` timestamp.
    pub updated_at: Option<TimeRange>,
    /// Time range for filtering on the `closed_at` timestamp.
    pub closed_at: Option<TimeRange>,
}
