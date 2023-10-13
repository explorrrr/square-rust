//! OrderFulfillmentUpdatedUpdate

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::fulfillment_state::FulfillmentStateV20230925;

/// Information about fulfillment updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderFulfillmentUpdatedUpdateV20230925 {
    /// A unique ID that identifies the fulfillment only within this order.
    pub fulfillment_uid: Option<String>,
    /// The state of the fulfillment before the change. The state is not populated if the fulfillment is created with this new Order version.
    pub old_state: Option<FulfillmentStateV20230925>,
    /// The state of the fulfillment after the change. The state might be equal to old_state if a non-state field was changed on the fulfillment (such as the tracking number).
    pub new_state: Option<FulfillmentStateV20230925>,
}
