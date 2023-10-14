//! SquareAccountDetails

use serde::{Deserialize, Serialize};

use crate::api::models::objects::error::SquareError;

/// Additional details about Square Account payments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SquareAccountDetailsV20230925 {
    /// Unique identifier for the payment source used for this payment.
    ///
    /// Max Length 255
    pub payment_source_token: Option<String>,
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<SquareError>>,
}
