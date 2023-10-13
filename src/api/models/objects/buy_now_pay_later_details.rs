//! BuyNowPayLaterDetails

use serde::{Deserialize, Serialize};

/// Additional details about a Buy Now Pay Later payment type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyNowPayLaterDetails {
    /// The brand used for the Buy Now Pay Later payment. The brand can be AFTERPAY, CLEARPAY or UNKNOWN.
    /// Max Length 50
    pub brand: Option<String>,
    /// Details about an Afterpay payment. These details are only populated if the brand is AFTERPAY.
    pub afterpay_details: Option<AfterpayDetails>,
    /// Details about a Clearpay payment. These details are only populated if the brand is CLEARPAY.
    pub clearpay_details: Option<ClearpayDetails>,
}
