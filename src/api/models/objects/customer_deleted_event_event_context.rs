//! CustomerDeletedEventEventContext

use serde::{Deserialize, Serialize};

/// Information about the change that triggered the event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerDeletedEventEventContext {
    /// Information about the merge operation associated with the event.
    pub merge: Option<CustomerDeletedEventEventContextMerge>,
}
