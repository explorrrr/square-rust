//! TenderBankAccountDetails

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::tender_bank_account_details_status::TenderBankAccountDetailsStatusV20230925;

/// Represents the details of a tender with type BANK_ACCOUNT.
///
/// See [BankAccountPaymentDetails](https://developer.squareup.com/reference/square/objects/BankAccountPaymentDetails) for more exposed details of a bank account payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenderBankAccountDetailsV20230925 {
    /// The bank account payment's current state.
    pub status: Option<TenderBankAccountDetailsStatusV20230925>,
}
