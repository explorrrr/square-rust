//! InvoiceFilter

use serde::{Deserialize, Serialize};

/// Describes query filters to apply.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceFilterV20230925 {
    /// Limits the search to the specified locations. A location is required. In the current implementation, only one location can be specified.
    pub location_ids: Vec<String>,
    /// Limits the search to the specified customers, within the specified locations. Specifying a customer is optional. In the current implementation, a maximum of one customer can be specified.
    pub customer_ids: Option<Vec<String>>,
}
