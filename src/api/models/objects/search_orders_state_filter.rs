//! SearchOrdersStateFilter

use serde::{Deserialize, Serialize};

/// Filter by the current order state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOrdersStateFilter {
    /// States to filter for.
    pub states: Vec<OrderState>,
}
