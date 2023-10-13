//! BulkDeleteOrderCustomAttributesRequestDeleteCustomAttribute

use serde::{Deserialize, Serialize};

/// Represents one delete within the bulk operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkDeleteOrderCustomAttributesRequestDeleteCustomAttributeV20230925 {
    /// The key of the custom attribute to delete. This key must match the key of an existing custom attribute definition.
    /// Min Length 1
    pub key: Option<String>,
    /// The ID of the target order.
    /// Min Length 1
    /// Max Length 255
    pub order_id: String,
}
