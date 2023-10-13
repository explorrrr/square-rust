//! SubscriptionPricing

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::subscription_pricing_type::SubscriptionPricingTypeV20230925;

use super::money::MoneyV20230925;

/// Describes the pricing for the subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPricingV20230925 {
    /// RELATIVE or STATIC
    pub r#type: Option<SubscriptionPricingTypeV20230925>,
    /// The ids of the discount catalog objects
    pub discount_ids: Option<Vec<String>>,
    /// The price of the subscription, if STATIC
    pub price_money: Option<MoneyV20230925>,
}
