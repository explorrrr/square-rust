//! SubscriptionEvent

use serde::{Deserialize, Serialize};

/// Describes changes to a subscription and the subscription status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionEvent {
    /// The ID of the subscription event.
    pub id: String,
    /// Type of the subscription event.
    pub subscription_event_type: String,
    /// The YYYY-MM-DD-formatted date (for example, 2013-01-15) when the subscription event occurred.
    pub effective_date: String,
    /// Additional information about the subscription event.
    pub info: Option<SubscriptionEventInfo>,
    /// A list of Phases, to pass phase-specific information used in the swap.
    pub phases: Option<Vec<Phase>>,
    /// The ID of the subscription plan variation associated with the subscription.
    pub plan_variation_id: String,
}
