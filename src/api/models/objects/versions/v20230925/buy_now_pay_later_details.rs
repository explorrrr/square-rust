//! BuyNowPayLaterDetails

use serde::{Deserialize, Serialize};

use crate::api::models::objects::versions::v20230925::afterpay_details::AfterpayDetailsV20230925;
use crate::api::models::objects::versions::v20230925::clearpay_details::ClearpayDetailsV20230925;

/// Additional details about a Buy Now Pay Later payment type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyNowPayLaterDetailsV20230925 {
    /// The brand used for the Buy Now Pay Later payment. The brand can be AFTERPAY, CLEARPAY or UNKNOWN.
    /// Max Length 50
    pub brand: Option<String>,
    /// Details about an Afterpay payment. These details are only populated if the brand is AFTERPAY.
    pub afterpay_details: Option<AfterpayDetailsV20230925>,
    /// Details about a Clearpay payment. These details are only populated if the brand is CLEARPAY.
    pub clearpay_details: Option<ClearpayDetailsV20230925>,
}
