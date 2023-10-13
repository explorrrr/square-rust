//! SearchOrdersSort

use serde::{Deserialize, Serialize};

/// Sorting criteria for a SearchOrders request.
///
/// Results can only be sorted by a timestamp field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOrdersSort {
    /// The field to sort by.
    ///
    /// **Important**: When using a [DateTimeFilter](https://developer.squareup.com/reference/square/objects/SearchOrdersFilter), sort_field must match the timestamp field that the DateTimeFilter uses to filter. For example, if you set your sort_field to CLOSED_AT and you use a DateTimeFilter, your DateTimeFilter must filter for orders by their CLOSED_AT date. If this field does not match the timestamp field in DateTimeFilter, SearchOrders returns an error.
    ///
    /// Default: CREATED_AT.
    pub sort_field: SearchOrdersSortField,
    /// The chronological order in which results are returned. Defaults to DESC.
    pub sort_order: Option<SortOrder>,
}
