//! SubscriptionPricingType Enum

use serde::{Deserialize, Serialize};

/// Determines the pricing of a [Subscription](https://developer.squareup.com/reference/square/objects/Subscription)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionPricingType {
    /// Static pricing
    Static,
    /// Relative pricing
    Relative,
}
