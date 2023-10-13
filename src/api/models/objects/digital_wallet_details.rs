//! DigitalWalletDetails

use serde::{Deserialize, Serialize};

/// Additional details about WALLET type payments.
///
/// Contains only non-confidential information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalWalletDetails {
    /// The status of the WALLET payment. The status can be AUTHORIZED, CAPTURED, VOIDED, or FAILED.
    /// Max Length 50
    pub status: Option<String>,
    /// The brand used for the WALLET payment. The brand can be CASH_APP, PAYPAY or UNKNOWN.
    /// Max Length 50
    pub brand: Option<String>,
    /// Brand-specific details for payments with the brand of CASH_APP.
    pub cash_app_details: Option<CashAppDetails>,
}
