//! LoyaltyEventLoyaltyAccountFilter

use serde::{Deserialize, Serialize};

/// Filter events by loyalty account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventLoyaltyAccountFilterV20230925 {
    /// The ID of the [loyalty account](https://developer.squareup.com/reference/square/objects/LoyaltyAccount) associated with loyalty events.
    ///
    /// Min Length 1
    pub loyalty_account_id: String,
}
