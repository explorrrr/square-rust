//! TenderSquareAccountDetails

use serde::{Deserialize, Serialize};

/// Represents the details of a tender with type SQUARE_ACCOUNT.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenderSquareAccountDetails {
    /// The Square Account payment's current state (such as AUTHORIZED or CAPTURED). See [TenderSquareAccountDetailsStatus](https://developer.squareup.com/reference/square/objects/TenderSquareAccountDetailsStatus) for possible values.
    pub status: Option<TenderSquareAccountDetailsStatus>,
}
