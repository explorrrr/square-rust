//! SearchSubscriptionsQuery

use serde::{Deserialize, Serialize};

use super::search_subscriptions_filter::SearchSubscriptionsFilterV20230925;

/// Represents a query, consisting of specified query expressions, used to search for subscriptions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSubscriptionsQueryV20230925 {
    /// A list of query expressions.
    pub filter: Option<SearchSubscriptionsFilterV20230925>,
}
