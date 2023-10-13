//! OrderReturnTax

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::{order_line_item_tax_type::OrderLineItemTaxTypeV20230925, order_line_item_tax_scope::OrderLineItemTaxScopeV20230925};

use super::money::MoneyV20230925;

/// Represents a tax being returned that applies to one or more return line items in an order.
///
/// Fixed-amount, order-scoped taxes are distributed across all non-zero return line item totals. The amount distributed to each return line item is relative to that item’s contribution to the order subtotal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderReturnTaxV20230925 {
    /// A unique ID that identifies the returned tax only within this order.
    ///
    /// Max Length 60
    pub uid: Option<String>,
    /// The tax uid from the order that contains the original tax charge.
    ///
    /// Max Length 60
    pub source_tax_uid: Option<String>,
    /// The catalog object ID referencing [CatalogTax](https://developer.squareup.com/reference/square/objects/CatalogTax).
    ///
    /// Max Length 192
    pub catalog_object_id: Option<String>,
    /// The version of the catalog object that this tax references.
    pub catalog_version: Option<i64>,
    /// The tax's name.
    ///
    /// Max Length 255
    pub name: Option<String>,
    /// Indicates the calculation method used to apply the tax.
    pub r#type: Option<OrderLineItemTaxTypeV20230925>,
    /// The percentage of the tax, as a string representation of a decimal number. For example, a value of `"7.25"` corresponds to a percentage of 7.25%.
    ///
    /// Max Length 10
    pub percentage: Option<String>,
    /// The amount of money applied by the tax in an order.
    pub applied_money: Option<MoneyV20230925>,
    /// Indicates the level at which the OrderReturnTax applies. For ORDER scoped taxes, Square generates references in `applied_taxes` on all OrderReturnLineItems. For LINE_ITEM scoped taxes, the tax is only applied to OrderReturnLineItems with references in their `applied_discounts` field.
    pub scope: Option<OrderLineItemTaxScopeV20230925>,
}
