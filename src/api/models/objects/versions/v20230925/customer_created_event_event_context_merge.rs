//! CustomerCreatedEventEventContextMerge

use serde::{Deserialize, Serialize};

/// Information about a merge operation, which creates a new customer using aggregated properties from two or more existing customers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerCreatedEventEventContextMergeV20230925 {
    /// The IDs of the existing customers that were merged and then deleted.
    pub from_customer_ids: Option<Vec<String>>,
    /// The ID of the new customer created by the merge.
    pub to_customer_id: Option<String>,
}
