//! CustomerDeletedEventEventContext

use serde::{Deserialize, Serialize};

use super::customer_deleted_event_event_context_merge::CustomerDeletedEventEventContextMergeV20230925;

/// Information about the change that triggered the event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerDeletedEventEventContextV20230925 {
    /// Information about the merge operation associated with the event.
    pub merge: Option<CustomerDeletedEventEventContextMergeV20230925>,
}
