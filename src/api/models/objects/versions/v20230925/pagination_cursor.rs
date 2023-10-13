//! PaginationCursor

use serde::{Deserialize, Serialize};

/// Used internally to encapsulate pagination details.
///
/// The resulting proto will be base62 encoded in order to produce a cursor that can be used externally.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationCursorV20230925 {
    /// The ID of the last resource in the current page. The page can be in an ascending or descending order
    pub order_value: Option<String>,
}
