//! TenderSquareAccountDetailsStatus Enum

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TenderSquareAccountDetailsStatusV20230925 {
    /// The Square Account payment has been authorized but not yet captured.
    Authorized,
    /// The Square Account payment was authorized and subsequently captured (i.e., completed).
    Captured,
    /// The Square Account payment was authorized and subsequently voided (i.e., canceled).
    Voided,
    /// The Square Account payment failed.
    Failed,
}
