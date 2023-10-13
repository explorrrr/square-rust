//! TenderBuyNowPayLaterDetails

use serde::{Deserialize, Serialize};

/// Represents the details of a tender with type BUY_NOW_PAY_LATER.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenderBuyNowPayLaterDetails {
    /// The Buy Now Pay Later brand.
    pub buy_now_pay_later_brand: Option<TenderBuyNowPayLaterDetailsBrand>,
    /// The buy now pay later payment's current state (such as AUTHORIZED or CAPTURED). See [TenderBuyNowPayLaterDetailsStatus](https://developer.squareup.com/reference/square/objects/TenderBuyNowPayLaterDetailsStatus) for possible values.
    pub status: Option<TenderBuyNowPayLaterDetailsStatus>,
}
