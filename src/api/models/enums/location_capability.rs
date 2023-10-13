//! LocationCapability Enum

use serde::{Deserialize, Serialize};

/// The capabilities a location might have.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LocationCapability {
    /// The capability to process credit card transactions with Square.
    CreditCardProcessing,
    /// The capability to receive automatic transfers from Square.
    AutomaticTransfers,
    /// The capability to process unlinked refunds with Square.
    UnlinkedRefunds,
}
