//! SubscriptionPricing

use serde::{Deserialize, Serialize};

/// Describes the pricing for the subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPricing {
    /// RELATIVE or STATIC
    pub r#type: Option<SubscriptionPricingType>,
    /// The ids of the discount catalog objects
    pub discount_ids: Option<Vec<String>>,
    /// The price of the subscription, if STATIC
    pub price_money: Option<Money>,
}
