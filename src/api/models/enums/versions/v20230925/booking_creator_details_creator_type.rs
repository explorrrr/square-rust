//! BookingCreatorDetailsCreatorType Enum

use serde::{Deserialize, Serialize};

/// Supported types of a booking creator.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BookingCreatorDetailsCreatorTypeV20230925 {
    /// The creator is of the seller type.
    TeamMember,
    /// The creator is of the buyer type.
    Customer,
}
