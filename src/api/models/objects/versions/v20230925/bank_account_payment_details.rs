//! BankAccountPaymentDetails

use crate::api::models::objects::versions::v20230925::ach_details::ACHDetailsV20230925;
use crate::api::models::objects::error::SquareError;
use crate::api::models::enums::versions::v20230925::country::CountryV20230925;

use serde::{Deserialize, Serialize};

/// Additional details about BANK_ACCOUNT type payments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccountPaymentDetailsV20230925 {
    /// The name of the bank associated with the bank account.
    /// Max Length 100
    pub bank_name: Option<String>,
    /// The type of the bank transfer. The type can be ACH or UNKNOWN.
    /// Max Length 50
    pub transfer_type: Option<String>,
    /// The ownership type of the bank account performing the transfer. The type can be INDIVIDUAL, COMPANY, or ACCOUNT_TYPE_UNKNOWN.
    /// Max Length 50
    pub account_ownership_type: Option<String>,
    /// Uniquely identifies the bank account for this seller and can be used to determine if payments are from the same bank account.
    /// Max Length 255
    pub fingerprint: Option<String>,
    /// The two-letter ISO code representing the country the bank account is located in.
    /// Min Length 2
    /// Max Length 2
    pub country: Option<CountryV20230925>,
    /// The statement description as sent to the bank.
    /// Max Length 1000
    pub statement_description: Option<String>,
    /// ACH-specific information about the transfer. The information is only populated if the transfer_type is ACH.
    pub ach_details: Option<ACHDetailsV20230925>,
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<SquareError>>,
}
