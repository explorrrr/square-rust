//! OrderEntry

use serde::{Deserialize, Serialize};

/// A lightweight description of an [order](https://developer.squareup.com/reference/square/objects/Order) that is returned when returned_entries is true on a [SearchOrdersRequest](https://developer.squareup.com/reference/square/orders-api/search-orders).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderEntry {
    /// The ID of the order.
    pub order_id: Option<String>,
    /// Read only The version number, which is incremented each time an update is committed to the order. Orders that were not created through the API do not include a version number and therefore cannot be updated.
    ///
    /// [Read more about working with versions](https://developer.squareup.com/docs/orders-api/manage-orders/update-orders).
    pub version: Option<i32>,
    /// The location ID the order belongs to.
    pub location_id: Option<String>,
}
