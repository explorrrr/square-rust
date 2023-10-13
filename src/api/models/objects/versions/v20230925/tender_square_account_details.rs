//! TenderSquareAccountDetails

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::tender_square_account_details_status::TenderSquareAccountDetailsStatusV20230925;

/// Represents the details of a tender with type SQUARE_ACCOUNT.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenderSquareAccountDetailsV20230925 {
    /// The Square Account payment's current state (such as AUTHORIZED or CAPTURED). See [TenderSquareAccountDetailsStatus](https://developer.squareup.com/reference/square/objects/TenderSquareAccountDetailsStatus) for possible values.
    pub status: Option<TenderSquareAccountDetailsStatusV20230925>,
}
