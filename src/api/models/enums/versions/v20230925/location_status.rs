//! LocationStatus Enum

use serde::{Deserialize, Serialize};

/// A location's status.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LocationStatusV20230925 {
    /// The location is active for business.
    Active,
    /// The location is not active for business. Inactive locations provide historical information. Hide inactive locations unless the user has requested to see them.
    Inactive,
}
