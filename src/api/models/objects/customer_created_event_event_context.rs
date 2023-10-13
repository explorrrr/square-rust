//! CustomerCreatedEventEventContext

use serde::{Deserialize, Serialize};

/// Provides information about a `CUSTOMER_CREATED` event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerCreatedEventEventContext {
    /// Provides information about the merge operation that was completed.
    pub merge: Option<CustomerCreatedEventEventContextMerge>,
}
