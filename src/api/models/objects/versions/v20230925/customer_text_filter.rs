//! CustomerTextFilter

use serde::{Deserialize, Serialize};

/// A filter to select customers based on exact or fuzzy matching of customer attributes against a specified query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerTextFilterV20230925 {
    /// Use the exact filter to select customers whose attributes match exactly the specified query.
    pub exact: Option<String>,
    /// Use the fuzzy filter to select customers whose attributes match the specified query in a fuzzy manner. When the fuzzy option is used, search queries are tokenized, and then each query token must be matched somewhere in the searched attribute. For single token queries, this is effectively the same behavior as a partial match operation.
    pub fuzzy: Option<String>,
}
