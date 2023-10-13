//! LoyaltyRewardStatus Enum

use serde::{Deserialize, Serialize};

/// The status of the loyalty reward.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoyaltyRewardStatus {
    /// The reward is issued.
    Issued,
    /// The reward is redeemed.
    Redeemed,
    /// The reward is deleted.
    Deleted,
}
