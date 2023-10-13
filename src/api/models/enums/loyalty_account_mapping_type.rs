//! LoyaltyAccountMappingType Enum

use serde::{Deserialize, Serialize};

/// The type of mapping.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoyaltyAccountMappingType {
    /// The loyalty account is mapped by phone.
    Phone,
}
