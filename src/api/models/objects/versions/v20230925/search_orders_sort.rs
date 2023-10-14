//! SearchOrdersSort

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::{
    search_orders_sort_field::SearchOrdersSortFieldV20230925, sort_order::SortOrderV20230925,
};

/// Sorting criteria for a SearchOrders request.
///
/// Results can only be sorted by a timestamp field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOrdersSortV20230925 {
    /// The field to sort by.
    ///
    /// **Important**: When using a [DateTimeFilter](https://developer.squareup.com/reference/square/objects/SearchOrdersFilter), sort_field must match the timestamp field that the DateTimeFilter uses to filter. For example, if you set your sort_field to CLOSED_AT and you use a DateTimeFilter, your DateTimeFilter must filter for orders by their CLOSED_AT date. If this field does not match the timestamp field in DateTimeFilter, SearchOrders returns an error.
    ///
    /// Default: CREATED_AT.
    pub sort_field: SearchOrdersSortFieldV20230925,
    /// The chronological order in which results are returned. Defaults to DESC.
    pub sort_order: Option<SortOrderV20230925>,
}
