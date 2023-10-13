//! PaymentLinkRelatedResources

use serde::{Deserialize, Serialize};

use super::catalog_object::CatalogObjectV20230925;
use crate::api::models::objects::versions::v20230925::order::OrderV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentLinkRelatedResourcesV20230925 {
    /// The order associated with the payment link.
    orders: Option<Vec<OrderV20230925>>,
    /// The subscription plan associated with the payment link.
    subscription_plans: Option<Vec<CatalogObjectV20230925>>,
}
