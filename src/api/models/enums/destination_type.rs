//! DestinationType Enum

use serde::{Deserialize, Serialize};

/// List of possible destinations against which a payout can be made.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DestinationType {
    /// An external bank account outside of Square.
    BankAccount,
    /// An external card outside of Square used for the transfer.
    Card,
    /// Square Checking or Savings account (US), Square Card (CA)
    SquareStoredBalance,
}
