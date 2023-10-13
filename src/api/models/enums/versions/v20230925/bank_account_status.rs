//! BankAccountStatus Enum

use serde::{Deserialize, Serialize};

/// Indicates the current verification status of a BankAccount object.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BankAccountStatusV20230925 {
    /// Indicates that the verification process has started. Some features (for example, creditable or debitable) may be provisionally enabled on the bank account.
    VerificationInProgress,
    /// Indicates that the bank account was successfully verified.
    Verified,
    /// Indicates that the bank account is disabled and is permanently unusable for funds transfer. A bank account can be disabled because of a failed verification attempt or a failed deposit attempt.
    Disabled,
}
