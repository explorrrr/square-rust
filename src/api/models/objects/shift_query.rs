//! ShiftQuery

use serde::{Deserialize, Serialize};

/// The parameters of a Shift search query, which includes filter and sort options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftQuery {
    /// Query filter options.
    pub filter: Option<ShiftFilter>,
    /// Sort order details.
    pub sort: Option<ShiftSort>,
}
