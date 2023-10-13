//! SubscriptionAction

use serde::{Deserialize, Serialize};

/// Represents an action as a pending change to a subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionAction {
    /// The ID of an action scoped to a subscription.
    pub id: Option<String>,
    /// The type of the action.
    pub r#type: Option<SubscriptionActionType>,
    /// The YYYY-MM-DD-formatted date when the action occurs on the subscription.
    pub effective_date: Option<String>,
    /// A list of Phases, to pass phase-specific information used in the swap.
    pub phases: Option<Vec<Phase>>,
    /// The target subscription plan variation that a subscription switches to, for a SWAP_PLAN action.
    pub new_plan_variation_id: Option<String>,
}
