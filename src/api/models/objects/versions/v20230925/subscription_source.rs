//! SubscriptionSource

use serde::{Deserialize, Serialize};

/// The origination details of the subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionSourceV20230925 {
    /// The name used to identify the place (physical or digital) that a subscription originates. If unset, the name defaults to the name of the application that created the subscription.
    ///
    /// Max Length 255
    pub name: Option<String>,
}
