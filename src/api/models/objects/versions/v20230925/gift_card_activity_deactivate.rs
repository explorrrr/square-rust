//! GiftCardActivityDeactivate

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::gift_card_activity_deactivate_reason::GiftCardActivityDeactivateReasonV20230925;

/// Represents details about a DEACTIVATE [gift card activity type](https://developer.squareup.com/reference/square/objects/GiftCardActivityType).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardActivityDeactivateV20230925 {
    /// The reason the gift card was deactivated.
    pub reason: GiftCardActivityDeactivateReasonV20230925,
}
