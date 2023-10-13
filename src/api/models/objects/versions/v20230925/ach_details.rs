//! ACHDetails

use serde::{Deserialize, Serialize};

/// ACH-specific details about BANK_ACCOUNT type payments with the transfer_type of ACH.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACHDetailsV20230925 {
    /// The routing number for the bank account.
    /// Max Length: 50
    pub routing_number: Option<String>,
    /// The last few digits of the bank account number.
    /// Min Length: 1 Max Length: 4
    pub account_number_suffix: Option<String>,
    /// The type of the bank account performing the transfer. The account type can be CHECKING, SAVINGS, or UNKNOWN.
    /// Max Length: 50
    pub account_type: Option<String>,
}
