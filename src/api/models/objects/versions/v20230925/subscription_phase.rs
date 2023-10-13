//! SubscriptionPhase

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::subscription_cadence::SubscriptionCadenceV20230925;

use super::{money::MoneyV20230925, subscription_pricing::SubscriptionPricingV20230925};

/// Describes a phase in a subscription plan variation.
///
/// For more information, see [Subscription Plans and Variations](https://developer.squareup.com/docs/subscriptions-api/plans-and-variations).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPhaseV20230925 {
    /// The Square-assigned ID of the subscription phase. This field cannot be changed after a `SubscriptionPhase` is created.
    pub uid: Option<String>,
    /// The billing cadence of the phase. For example, weekly or monthly. This field cannot be changed after a `SubscriptionPhase` is created.
    pub cadence: SubscriptionCadenceV20230925,
    /// The number of cadences the phase lasts. If not set, the phase never ends. Only the last phase can be indefinite. This field cannot be changed after a `SubscriptionPhase` is created.
    pub periods: Option<i32>,
    /// The amount to bill for each cadence. Failure to specify this field results in a `MISSING_REQUIRED_PARAMETER` error at runtime.
    pub recurring_price_money: Option<MoneyV20230925>,
    /// The position this phase appears in the sequence of phases defined for the plan, indexed from 0. This field cannot be changed after a `SubscriptionPhase` is created.
    pub ordinal: Option<i64>,
    /// The subscription pricing.
    pub pricing: Option<SubscriptionPricingV20230925>,
}
