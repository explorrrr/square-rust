//! SearchOrdersSourceFilter

use serde::{Deserialize, Serialize};

/// A filter based on order source information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOrdersSourceFilterV20230925 {
    /// Filters by the [Source](https://developer.squareup.com/reference/square/objects/OrderSource) name. The filter returns any orders with a source.name that matches any of the listed source names.
    ///
    /// Max: 10 source names.
    pub source_names: Option<Vec<String>>,
}
