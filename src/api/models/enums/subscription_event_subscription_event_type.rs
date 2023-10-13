//! SubscriptionEventSubscriptionEventType Enum

use serde::{Deserialize, Serialize};

/// Supported types of an event occurred to a subscription.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionEventSubscriptionEventType {
    /// The subscription was started.
    StartSubscription,
    /// The subscription plan was changed.
    PlanChange,
    /// The subscription was stopped.
    StopSubscription,
    /// The subscription was deactivated
    DeactivateSubscription,
    /// The subscription was resumed.
    ResumeSubscription,
    /// The subscription was paused.
    PauseSubscription,
}
