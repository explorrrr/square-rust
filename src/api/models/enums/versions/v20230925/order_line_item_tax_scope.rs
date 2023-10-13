//! OrderLineItemTaxScope Enum

use serde::{Deserialize, Serialize};

/// Indicates whether this is a line-item or order-level tax.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderLineItemTaxScopeV20230925 {
    /// Used for reporting only. The original transaction tax scope is currently not supported by the API.
    OtherTaxScope,
    /// The tax should be applied only to line items specified by the OrderLineItemAppliedTax reference records.
    LineItem,
    /// The tax should be applied to the entire order.
    Order,
}
