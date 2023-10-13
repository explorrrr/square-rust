//! OrderLineItemAppliedDiscount

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;

/// Represents an applied portion of a discount to a line item in an order.
///
/// Order scoped discounts have automatically applied discounts present for each line item. Line-item scoped discounts must have applied discounts added manually for any applicable line items. The corresponding applied money is automatically computed based on participating line items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLineItemAppliedDiscountV20230925 {
    /// A unique ID that identifies the applied discount only within this order.
    ///
    /// Max Length 60
    pub uid: Option<String>,
    /// The uid of the discount that the applied discount represents. It must reference a discount present in the order.discounts field.
    ///
    /// This field is immutable. To change which discounts apply to a line item, you must delete the discount and re-add it as a new OrderLineItemAppliedDiscount.
    ///
    /// Min Length 1 Max Length 60
    pub discount_uid: String,
    /// Read only The amount of money applied by the discount to the line item.
    pub applied_money: Option<MoneyV20230925>,
}
