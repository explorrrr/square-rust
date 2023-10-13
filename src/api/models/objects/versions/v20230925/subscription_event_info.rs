//! SubscriptionEventInfo

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::subscription_event_info_code::SubscriptionEventInfoCodeV20230925;

/// Provides information about the subscription event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionEventInfoV20230925 {
    /// A human-readable explanation for the event.
    pub detail: Option<String>,
    /// An info code indicating the subscription event that occurred.
    pub code: Option<SubscriptionEventInfoCodeV20230925>,
}
