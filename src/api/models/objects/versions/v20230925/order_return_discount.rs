//! OrderReturnDiscount

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;
use crate::api::models::enums::versions::v20230925::{
    order_line_item_discount_scope::OrderLineItemDiscountScopeV20230925,
    order_line_item_discount_type::OrderLineItemDiscountTypeV20230925,
};

/// Represents a discount being returned that applies to one or more return line items in an order.
///
/// Fixed-amount, order-scoped discounts are distributed across all non-zero return line item totals. The amount distributed to each return line item is relative to that item’s contribution to the order subtotal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderReturnDiscountV20230925 {
    /// A unique ID that identifies the returned discount only within this order.
    ///
    /// Max Length 60
    pub uid: Option<String>,
    /// The discount uid from the order that contains the original application of this discount.
    ///
    /// Max Length 60
    pub source_discount_uid: Option<String>,
    /// The catalog object ID referencing [CatalogDiscount](https://developer.squareup.com/reference/square/objects/CatalogDiscount).
    ///
    /// Max Length 192
    pub catalog_object_id: Option<String>,
    /// The version of the catalog object that this discount references.
    pub catalog_version: Option<i64>,
    /// The discount's name.
    ///
    /// Max Length 255
    pub name: Option<String>,
    /// The type of the discount. If it is created by the API, it is `FIXED_PERCENTAGE` or `FIXED_AMOUNT`.
    ///
    /// Discounts that do not reference a catalog object ID must have a type of `FIXED_PERCENTAGE` or `FIXED_AMOUNT`.
    pub r#type: Option<OrderLineItemDiscountTypeV20230925>,
    /// The percentage of the tax, as a string representation of a decimal number. A value of `"7.25"` corresponds to a percentage of 7.25%.
    ///
    /// `percentage` is not set for amount-based discounts.
    ///
    /// Max Length 10
    pub percentage: Option<String>,
    /// The amount of discount actually applied to this line item. When an amount-based discount is at the order level, this value is different from `amount_money` because the discount is distributed across the line items.
    ///
    /// `amount_money` is not set for percentage-based discounts.
    pub amount_money: Option<MoneyV20230925>,
    /// The amount of discount actually applied to this line item in item. When an amount-based discount is at the order level, this value is different from `amount_money` because the discount is distributed across the line items.
    pub applied_money: Option<MoneyV20230925>,
    /// Indicates the level at which the OrderReturnDiscount applies. For ORDER scoped discounts, the server generates references in `applied_discounts` on all OrderReturnLineItems. For LINE_ITEM scoped discounts, the discount is only applied to OrderReturnLineItems with references in their `applied_discounts` field.
    pub scope: Option<OrderLineItemDiscountScopeV20230925>,
}
