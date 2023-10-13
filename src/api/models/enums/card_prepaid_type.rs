//! CardPrepaidType enum

use serde::{Deserialize, Serialize};

/// Indicates a card's prepaid type, such as NOT_PREPAID or PREPAID.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardPrepaidType {
    /// A card prepaid type not on this list.
    UnknownPrepaidType,
    /// NOT_PREPAID.
    NotPrepaid,
    /// PREPAID.
    Prepaid,
}
