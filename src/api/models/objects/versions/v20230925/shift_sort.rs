//! ShiftSort

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::{
    shift_sort_field::ShiftSortFieldV20230925, sort_order::SortOrderV20230925,
};

/// Sets the sort order of search results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftSortV20230925 {
    /// The field to sort on.
    pub field: Option<ShiftSortFieldV20230925>,
    /// The order in which results are returned. Defaults to DESC.
    pub order: Option<SortOrderV20230925>,
}
