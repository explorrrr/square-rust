//! SearchOrdersFulfillmentFilter

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::{fulfillment_type::FulfillmentTypeV20230925, fulfillment_state::FulfillmentStateV20230925};

/// Filter based on order fulfillment information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOrdersFulfillmentFilterV20230925 {
    /// A list of [fulfillment types](https://developer.squareup.com/reference/square/objects/FulfillmentType) to filter for. The list returns orders if any of its fulfillments match any of the fulfillment types listed in this field.
    pub fulfillment_types: Option<Vec<FulfillmentTypeV20230925>>,
    /// A list of [fulfillment states](https://developer.squareup.com/reference/square/objects/FulfillmentState) to filter for. The list returns orders if any of its fulfillments match any of the fulfillment states listed in this field.
    pub fulfillment_states: Option<Vec<FulfillmentStateV20230925>>,
}
