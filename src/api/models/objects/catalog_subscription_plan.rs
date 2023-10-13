//! CatalogSubscriptionPlan

use serde::{Deserialize, Serialize};

/// Describes a subscription plan.
///
/// A subscription plan represents what you want to sell in a subscription model, and includes references to each of the associated subscription plan variations. For more information, see [Subscription Plans and Variations](https://developer.squareup.com/docs/subscriptions-api/plans-and-variations).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogSubscriptionPlan {
    /// The name of the plan.
    pub name: String,
    /// A list of SubscriptionPhase containing the [SubscriptionPhase](https://developer.squareup.com/reference/square/objects/SubscriptionPhase) for this plan. This field it required. Not including this field will throw a REQUIRED_FIELD_MISSING error.
    pub phases: Option<Vec<SubscriptionPhase>>,
    /// The list of subscription plan variations available for this product.
    pub subscription_plan_variations: Option<Vec<CatalogObject>>,
    /// The list of IDs of CatalogItems that are eligible for subscription by this SubscriptionPlan's variations.
    pub eligible_item_ids: Option<Vec<String>>,
    /// The list of IDs of CatalogCategory that are eligible for subscription by this SubscriptionPlan's variations.
    pub eligible_category_ids: Option<Vec<String>>,
    /// If true, all items in the merchant's catalog are subscribable by this SubscriptionPlan.
    pub all_items: Option<bool>,
}
