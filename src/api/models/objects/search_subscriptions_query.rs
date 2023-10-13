//! SearchSubscriptionsQuery

use serde::{Deserialize, Serialize};

/// Represents a query, consisting of specified query expressions, used to search for subscriptions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSubscriptionsQuery {
    /// A list of query expressions.
    pub filter: Option<SearchSubscriptionsFilter>,
}
