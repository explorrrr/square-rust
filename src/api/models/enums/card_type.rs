//! CardType

use serde::{Deserialize, Serialize};

/// Indicates a card's type, such as CREDIT or DEBIT.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardType {
    /// Unknown card type
    UnknownCardType,
    /// Credit card
    Credit,
    /// Debit card
    Debit,
}
