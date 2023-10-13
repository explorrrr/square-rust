//! OrderUpdated

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::order_state::OrderStateV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderUpdatedV20230925 {
    /// The order's unique ID.
    pub order_id: Option<String>,
    /// The version number, which is incremented each time an update is committed to the order. Orders that were not created through the API do not include a version number and therefore cannot be updated.
    ///
    /// Read more about working with versions.
    pub version: Option<i32>,
    /// The ID of the seller location that this order is associated with.
    pub location_id: Option<String>,
    /// The state of the order.
    pub state: Option<OrderStateV20230925>,
    /// Read only The timestamp for when the order was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// Read only The timestamp for when the order was last updated, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
}
