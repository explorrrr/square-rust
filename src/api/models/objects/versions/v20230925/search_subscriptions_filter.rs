//! SearchSubscriptionsFilter

use serde::{Deserialize, Serialize};

/// Represents a set of query expressions (filters) to narrow the scope of targeted subscriptions returned by the [SearchSubscriptions](https://developer.squareup.com/reference/square/subscriptions-api/search-subscriptions) endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSubscriptionsFilterV20230925 {
    /// A filter to select subscriptions based on the subscribing customer IDs.
    pub customer_ids: Option<Vec<String>>,
    /// A filter to select subscriptions based on the location.
    pub location_ids: Option<Vec<String>>,
    /// A filter to select subscriptions based on the source application.
    pub source_names: Option<Vec<String>>,
}
