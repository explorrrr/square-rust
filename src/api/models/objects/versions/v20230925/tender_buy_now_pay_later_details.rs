//! TenderBuyNowPayLaterDetails

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::{tender_buy_now_pay_later_details_status::TenderBuyNowPayLaterDetailsStatusV20230925, tender_buy_now_pay_later_details_brand::TenderBuyNowPayLaterDetailsBrandV20230925};

/// Represents the details of a tender with type BUY_NOW_PAY_LATER.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenderBuyNowPayLaterDetailsV20230925 {
    /// The Buy Now Pay Later brand.
    pub buy_now_pay_later_brand: Option<TenderBuyNowPayLaterDetailsBrandV20230925>,
    /// The buy now pay later payment's current state (such as AUTHORIZED or CAPTURED). See [TenderBuyNowPayLaterDetailsStatus](https://developer.squareup.com/reference/square/objects/TenderBuyNowPayLaterDetailsStatus) for possible values.
    pub status: Option<TenderBuyNowPayLaterDetailsStatusV20230925>,
}
