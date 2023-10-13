//! SquareAccountDetails

use serde::{Deserialize, Serialize};

/// Additional details about Square Account payments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SquareAccountDetails {
    /// Unique identifier for the payment source used for this payment.
    ///
    /// Max Length 255
    pub payment_source_token: Option<String>,
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<SquareError>>,
}
