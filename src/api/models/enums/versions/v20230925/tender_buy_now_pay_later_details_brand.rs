//! TenderBuyNowPayLaterDetailsBrand Enum

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TenderBuyNowPayLaterDetailsBrandV20230925 {
    /// The Afterpay brand.
    Afterpay,
    /// The other brand.
    OtherBrand,
}