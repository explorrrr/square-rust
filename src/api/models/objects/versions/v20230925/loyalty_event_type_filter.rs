//! LoyaltyEventTypeFilter

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::loyalty_event_type::LoyaltyEventTypeV20230925;

/// Filter events by event type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventTypeFilterV20230925 {
    /// The loyalty event types used to filter the result. If multiple values are specified, the endpoint uses a logical OR to combine them.
    pub types: Vec<LoyaltyEventTypeV20230925>,
}
