//! CustomerInclusionExclusion Enum

use serde::{Deserialize, Serialize};

/// Indicates whether customers should be included in, or excluded from, the result set when they match the filtering criteria.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomerInclusionExclusionV20230925 {
    /// Customers should be included in the result set when they match the filtering criteria.
    Include,
    /// Customers should be excluded from the result set when they match the filtering criteria.
    Exclude,
}
