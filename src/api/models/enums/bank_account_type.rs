//! BankAccountType Enum

use serde::{Deserialize, Serialize};

/// Indicates the financial purpose of the bank account.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BankAccountType {
    /// An account at a financial institution against which checks can be drawn by the account depositor.
    Checking,
    /// An account at a financial institution that pays interest but cannot be used directly as money in the narrow sense of a medium of exchange.
    Savings,
    /// An account at a financial institution that contains a deposit of funds and/or securities.
    Investment,
    /// An account at a financial institution which cannot be described by the other types.
    Other,
    /// An account at a financial institution against which checks can be drawn specifically for business purposes (non-personal use).
    BusinessChecking,
}
