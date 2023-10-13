//! CatalogSubScriptionPlanVariation

use serde::{Deserialize, Serialize};

/// Describes a subscription plan variation.
///
/// A subscription plan variation represents how the subscription for a product or service is sold. For more information, see [Subscription Plans and Variations](https://developer.squareup.com/docs/subscriptions-api/plans-and-variations).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogSubscriptionPlanVariation {
    /// The name of the plan variation.
    pub name: String,
    /// A list containing each [SubscriptionPhase](https://developer.squareup.com/reference/square/objects/SubscriptionPhase) for this plan variation.
    pub phases: Vec<SubscriptionPhase>,
    /// The id of the subscription plan, if there is one.
    pub subscription_plan_id: Option<String>,
}
