//! AcceptedPaymentMethods

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptedPaymentMethodsV20230925 {
    /// Whether Apple Pay is accepted at checkout.
    pub apple_pay: Option<bool>,
    /// Whether Google Pay is accepted at checkout.
    pub google_pay: Option<bool>,
    /// Whether Cash App Pay is accepted at checkout.
    pub cash_app_pay: Option<bool>,
    /// Whether Afterpay/Clearpay is accepted at checkout.
    pub afterpay_clearpay: Option<bool>,
}
