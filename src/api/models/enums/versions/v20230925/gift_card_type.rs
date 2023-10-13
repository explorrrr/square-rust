//! GiftCardType Enum

use serde::{Deserialize, Serialize};

/// Indicates the gift card type.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GiftCardTypeV20230925 {
    /// A plastic gift card.
    Physical,
    /// A digital gift card.
    Digital,
}
