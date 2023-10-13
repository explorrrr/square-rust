//! SearchOrdersFilter

use serde::{Deserialize, Serialize};

use super::{search_orders_state_filter::SearchOrdersStateFilterV20230925, search_orders_date_time_filter::SearchOrdersDateTimeFilterV20230925, search_orders_fulfillment_filter::SearchOrdersFulfillmentFilterV20230925, search_orders_source_filter::SearchOrdersSourceFilterV20230925, search_orders_customer_filter::SearchOrdersCustomerFilterV20230925};

/// Filtering criteria to use for a SearchOrders request.
///
/// Multiple filters are ANDed together.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOrdersFilterV20230925 {
    /// Filter by [OrderState](https://developer.squareup.com/reference/square/objects/OrderState).
    pub state_filter: Option<SearchOrdersStateFilterV20230925>,
    /// Filter for results within a time range.
    ///
    /// Important: If you filter for orders by time range, you must set `SearchOrdersSort` to sort by the same field.
    /// [Learn more about filtering orders by time range](https://developer.squareup.com/docs/orders-api/manage-orders/search-orders#important-note-about-filtering-orders-by-time-range).
    pub date_time_filter: Option<SearchOrdersDateTimeFilterV20230925>,
    /// Filter by fulfillment type or state.
    pub fulfillment_filter: Option<SearchOrdersFulfillmentFilterV20230925>,
    /// Filter by source of order.
    pub source_filter: Option<SearchOrdersSourceFilterV20230925>,
    /// Filter by customers associated with the order.
    pub customer_filter: Option<SearchOrdersCustomerFilterV20230925>,
}
