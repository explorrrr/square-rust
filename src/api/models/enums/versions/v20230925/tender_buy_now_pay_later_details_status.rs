//! TenderBuyNowPayLaterDetailsStatus Enum

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TenderBuyNowPayLaterDetailsStatusV20230925 {
    /// The buy now pay later payment has been authorized but not yet captured.
    Authorized,
    /// The buy now pay later payment was authorized and subsequently captured (i.e., completed).
    Captured,
    /// The buy now pay later payment was authorized and subsequently voided (i.e., canceled).
    Voided,
    /// The buy now pay later payment failed.
    Failed,
}
