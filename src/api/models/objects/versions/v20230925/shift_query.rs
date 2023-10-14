//! ShiftQuery

use serde::{Deserialize, Serialize};

use super::{shift_filter::ShiftFilterV20230925, shift_sort::ShiftSortV20230925};

/// The parameters of a Shift search query, which includes filter and sort options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftQueryV20230925 {
    /// Query filter options.
    pub filter: Option<ShiftFilterV20230925>,
    /// Sort order details.
    pub sort: Option<ShiftSortV20230925>,
}
