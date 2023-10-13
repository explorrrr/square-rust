//! GiftCardActivityUnblock

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::gift_card_activity_unblock_reason::GiftCardActivityUnblockReasonV20230925;

/// Represents details about an UNBLOCK [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityUnblockV20230925 {
    /// The reason the gift card was unblocked.
    pub reason: GiftCardActivityUnblockReasonV20230925,
}
