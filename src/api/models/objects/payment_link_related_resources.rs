//! PaymentLinkRelatedResources

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentLinkRelatedResources {
    /// The order associated with the payment link.
    orders: Option<Vec<Order>>,
    /// The subscription plan associated with the payment link.
    subscription_plans: Option<Vec<CatalogObject>>,
}
