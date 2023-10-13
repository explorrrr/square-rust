//! TenderBankAccountDetails

use serde::{Deserialize, Serialize};

/// Represents the details of a tender with type BANK_ACCOUNT.
///
/// See [BankAccountPaymentDetails](https://developer.squareup.com/reference/square/objects/BankAccountPaymentDetails) for more exposed details of a bank account payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenderBankAccountDetails {
    /// The bank account payment's current state.
    pub status: Option<TenderBankAccountDetailsStatus>,
}
