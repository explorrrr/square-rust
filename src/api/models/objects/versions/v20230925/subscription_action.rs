//! SubscriptionAction

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::subscription_action_type::SubscriptionActionTypeV20230925;

use super::phase::PhaseV20230925;

/// Represents an action as a pending change to a subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionActionV20230925 {
    /// The ID of an action scoped to a subscription.
    pub id: Option<String>,
    /// The type of the action.
    pub r#type: Option<SubscriptionActionTypeV20230925>,
    /// The YYYY-MM-DD-formatted date when the action occurs on the subscription.
    pub effective_date: Option<String>,
    /// A list of Phases, to pass phase-specific information used in the swap.
    pub phases: Option<Vec<PhaseV20230925>>,
    /// The target subscription plan variation that a subscription switches to, for a SWAP_PLAN action.
    pub new_plan_variation_id: Option<String>,
}
