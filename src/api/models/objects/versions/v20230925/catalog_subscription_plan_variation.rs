//! CatalogSubScriptionPlanVariation

use serde::{Deserialize, Serialize};

use crate::api::models::objects::versions::v20230925::subscription_phase::SubscriptionPhaseV20230925;

/// Describes a subscription plan variation.
///
/// A subscription plan variation represents how the subscription for a product or service is sold. For more information, see [Subscription Plans and Variations](https://developer.squareup.com/docs/subscriptions-api/plans-and-variations).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogSubscriptionPlanVariationV20230925 {
    /// The name of the plan variation.
    pub name: String,
    /// A list containing each [SubscriptionPhase](https://developer.squareup.com/reference/square/objects/SubscriptionPhase) for this plan variation.
    pub phases: Vec<SubscriptionPhaseV20230925>,
    /// The id of the subscription plan, if there is one.
    pub subscription_plan_id: Option<String>,
}
