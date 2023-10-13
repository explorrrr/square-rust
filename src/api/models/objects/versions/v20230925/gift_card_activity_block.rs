//! GiftCardActivityBlock

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::gift_card_activity_block_reason::GiftCardActivityBlockReasonV20230925;

/// Represents details about a BLOCK [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityBlockV20230925 {
    /// The reason the gift card was blocked.
    pub reason: GiftCardActivityBlockReasonV20230925,
}
