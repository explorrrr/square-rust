//! CardCoBrand enum

use serde::{Deserialize, Serialize};

/// Indicates the brand for a co-branded card.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardCoBrand {
    /// A card brand not on this list.
    Unknown,
    /// AFTERPAY.
    Afterpay,
    /// CLEARPAY.
    Clearpay,
}
