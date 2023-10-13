//! CustomerCreatedEventEventContext

use serde::{Deserialize, Serialize};

use super::customer_created_event_event_context_merge::CustomerCreatedEventEventContextMergeV20230925;

/// Provides information about a `CUSTOMER_CREATED` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerCreatedEventEventContextV20230925 {
    /// Provides information about the merge operation that was completed.
    pub merge: Option<CustomerCreatedEventEventContextMergeV20230925>,
}
