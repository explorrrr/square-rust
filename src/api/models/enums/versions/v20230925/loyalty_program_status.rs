//! LoyaltyProgramStatus Enum

use serde::{Deserialize, Serialize};

/// Indicates whether the program is currently active.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoyaltyProgramStatusV20230925 {
    /// The loyalty program does not have an active subscription. Loyalty API requests fail.
    Inactive,
    /// The program is fully functional. The program has an active subscription.
    Active,
}
