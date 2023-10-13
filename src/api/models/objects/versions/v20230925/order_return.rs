//! OrderReturn

use serde::{Deserialize, Serialize};

use super::{order_return_line_item::OrderReturnLineItemV20230925, order_return_service_charge::OrderReturnServiceChargeV20230925, order_return_tax::OrderReturnTaxV20230925, order_return_discount::OrderReturnDiscountV20230925, order_rounding_adjustment::OrderRoundingAdjustmentV20230925, order_money_amounts::OrderMoneyAmountsV20230925};

/// The set of line items, service charges, taxes, discounts, tips, and other items being returned in an order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderReturnV20230925 {
    /// A unique ID that identifies the return only within this order.
    ///
    /// Max Length 60
    pub uid: Option<String>,
    /// An order that contains the original sale of these return line items.
    ///
    /// This is unset for unlinked returns.
    pub source_order_id: Option<String>,
    /// A collection of line items that are being returned.
    pub return_line_items: Option<Vec<OrderReturnLineItemV20230925>>,
    /// A collection of service charges that are being returned.
    pub return_service_charges: Option<Vec<OrderReturnServiceChargeV20230925>>,
    /// A collection of references to taxes being returned for an order, including the total applied tax amount to be returned.
    ///
    /// The taxes must reference a top-level tax ID from the source order.
    pub return_taxes: Option<Vec<OrderReturnTaxV20230925>>,
    /// A collection of references to discounts being returned for an order, including the total applied discount amount to be returned.
    ///
    /// The discounts must reference a top-level discount ID from the source order.
    pub return_discounts: Option<Vec<OrderReturnDiscountV20230925>>,
    /// A positive or negative rounding adjustment to the total value being returned.
    ///
    /// Adjustments are commonly used to apply cash rounding when the minimum unit of the account is smaller than the lowest physical denomination of the currency.
    pub rounding_adjustment: Option<OrderRoundingAdjustmentV20230925>,
    /// An aggregate monetary value being returned by this return entry.
    pub return_amounts: Option<OrderMoneyAmountsV20230925>,
}
