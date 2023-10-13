//! SearchOrdersCustomerFilter

use serde::{Deserialize, Serialize};

/// A filter based on the order `customer_id` and any tender `customer_id` associated with the order.
///
/// It does not filter based on the [FulfillmentRecipient](https://developer.squareup.com/reference/square/objects/FulfillmentRecipient) `customer_id`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOrdersCustomerFilterV20230925 {
    /// A list of customer IDs to filter by.
    ///
    /// Max: 10 customer IDs.
    pub customer_ids: Option<Vec<String>>,
}
