//! SubscriptionEvent

use serde::{Deserialize, Serialize};

use super::{subscription_event_info::SubscriptionEventInfoV20230925, phase::PhaseV20230925};

/// Describes changes to a subscription and the subscription status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionEventV20230925 {
    /// The ID of the subscription event.
    pub id: String,
    /// Type of the subscription event.
    pub subscription_event_type: String,
    /// The YYYY-MM-DD-formatted date (for example, 2013-01-15) when the subscription event occurred.
    pub effective_date: String,
    /// Additional information about the subscription event.
    pub info: Option<SubscriptionEventInfoV20230925>,
    /// A list of Phases, to pass phase-specific information used in the swap.
    pub phases: Option<Vec<PhaseV20230925>>,
    /// The ID of the subscription plan variation associated with the subscription.
    pub plan_variation_id: String,
}
