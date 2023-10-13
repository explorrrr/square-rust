//! LoyaltyEventLocationFilter

use serde::{Deserialize, Serialize};

/// Filter events by location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventLocationFilterV20230925 {
    /// The [location](https://developer.squareup.com/reference/square/objects/Location) IDs for loyalty events to query. If multiple values are specified, the endpoint uses a logical OR to combine them.
    pub location_ids: Vec<String>,
}
