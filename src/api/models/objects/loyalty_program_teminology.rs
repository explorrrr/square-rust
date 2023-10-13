//! LoyaltyProgramTerminology

use serde::{Deserialize, Serialize};

/// Represents the naming used for loyalty points.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyProgramTerminology {
    /// A singular unit for a point (for example, 1 point is called 1 star).
    ///
    /// Min Length 1
    pub one: String,
    /// A plural unit for point (for example, 10 points is called 10 stars).
    ///
    /// Min Length 1
    pub other: String,
}
