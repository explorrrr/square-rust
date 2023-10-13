//! OrderSource

use serde::{Deserialize, Serialize};

/// Represents the origination details of an order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSourceV20230925 {
    /// The name used to identify the place (physical or digital) that an order originates. If unset, the name defaults to the name of the application that created the order.
    pub name: Option<String>,
}
