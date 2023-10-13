//! OrderLineItemPricingBlocklistsBlockedDiscount

use serde::{Deserialize, Serialize};

/// A discount to block from applying to a line item.
///
/// The discount must be identified by either discount_uid or discount_catalog_object_id, but not both.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLineItemPricingBlocklistsBlockedDiscountV20230925 {
    /// A unique ID of the BlockedDiscount within the order.
    ///
    /// Max Length 60
    pub uid: Option<String>,
    /// The uid of the discount that should be blocked. Use this field to block ad hoc discounts. For catalog discounts, use the discount_catalog_object_id field.
    ///
    /// Max Length 60
    pub discount_uid: Option<String>,
    /// The catalog_object_id of the discount that should be blocked. Use this field to block catalog discounts. For ad hoc discounts, use the discount_uid field.
    ///
    /// Max Length 192
    pub discount_catalog_object_id: Option<String>,
}
