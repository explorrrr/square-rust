//! SearchOrdersFulfillmentFilter

use serde::{Deserialize, Serialize};

/// Filter based on order fulfillment information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOrdersFulfillmentFilter {
    /// A list of [fulfillment types](https://developer.squareup.com/reference/square/objects/FulfillmentType) to filter for. The list returns orders if any of its fulfillments match any of the fulfillment types listed in this field.
    pub fulfillment_types: Option<Vec<FulfillmentType>>,
    /// A list of [fulfillment states](https://developer.squareup.com/reference/square/objects/FulfillmentState) to filter for. The list returns orders if any of its fulfillments match any of the fulfillment states listed in this field.
    pub fulfillment_states: Option<Vec<FulfillmentState>>,
}
