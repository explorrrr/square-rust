//! SearchOrdersStateFilter

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::order_state::OrderStateV20230925;

/// Filter by the current order state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOrdersStateFilterV20230925 {
    /// States to filter for.
    pub states: Vec<OrderStateV20230925>,
}
