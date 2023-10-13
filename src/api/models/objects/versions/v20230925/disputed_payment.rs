//! DisputedPayment

use serde::{Deserialize, Serialize};

/// The payment the cardholder disputed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputedPaymentV20230925 {
    /// Square-generated unique ID of the payment being disputed.
    ///
    /// Min Length 1 Max Length 192
    pub payment_id: Option<String>,
}
