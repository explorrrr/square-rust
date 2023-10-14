//! OrderLineItemPricingBlocklists

use serde::{Deserialize, Serialize};

use super::{
    order_line_item_pricing_blocklists_blocked_discount::OrderLineItemPricingBlocklistsBlockedDiscountV20230925,
    order_line_item_pricing_blocklists_blocked_tax::OrderLineItemPricingBlocklistsBlockedTaxV20230925,
};

/// Describes pricing adjustments that are blocked from automatic application to a line item.
///
/// For more information, see [Apply Taxes and Discounts](https://developer.squareup.com/docs/orders-api/apply-taxes-and-discounts).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLineItemPricingBlocklistsV20230925 {
    /// A list of discounts blocked from applying to the line item. Discounts can be blocked by the discount_uid (for ad hoc discounts) or the discount_catalog_object_id (for catalog discounts).
    pub blocked_discounts: Option<Vec<OrderLineItemPricingBlocklistsBlockedDiscountV20230925>>,
    /// A list of taxes blocked from applying to the line item. Taxes can be blocked by the tax_uid (for ad hoc taxes) or the tax_catalog_object_id (for catalog taxes).
    pub blocked_taxes: Option<Vec<OrderLineItemPricingBlocklistsBlockedTaxV20230925>>,
}
