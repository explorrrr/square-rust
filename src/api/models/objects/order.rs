//! Order

use serde::{Deserialize, Serialize};

/// Contains all information related to a single order to process with Square, including line items that specify the products to purchase.
///
/// Order objects also include information about any associated tenders, refunds, and returns.
///
/// All Connect V2 Transactions have all been converted to Orders including all associated itemization data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// The order's unique ID.
    pub id: Option<String>,
    /// The ID of the seller location that this order is associated with.
    ///
    /// Min Length 1
    pub location_id: String,
    /// A client-specified ID to associate an entity in another system with this order.
    ///
    /// Max Length 40
    pub reference_id: Option<String>,
    /// The origination details of the order.
    pub source: Option<OrderSource>,
    /// The ID of the [customer](https://developer.squareup.com/reference/square/objects/Customer) associated with the order.
    ///
    /// You should specify a customer_id on the order (or the payment) to ensure that transactions are reliably linked to customers. Omitting this field might result in the creation of new [instant profiles](https://developer.squareup.com/docs/customers-api/what-it-does#instant-profiles).
    ///
    /// Max Length 191
    pub customer_id: Option<String>,
    /// The line items included in the order.
    pub line_items: Option<Vec<OrderLineItem>>,
    /// The list of all taxes associated with the order.
    ///
    /// Taxes can be scoped to either ORDER or LINE_ITEM. For taxes with LINE_ITEM scope, an OrderLineItemAppliedTax must be added to each line item that the tax applies to. For taxes with ORDER scope, the server generates an OrderLineItemAppliedTax for every line item.
    ///
    /// On reads, each tax in the list includes the total amount of that tax applied to the order.
    ///
    /// IMPORTANT: If LINE_ITEM scope is set on any taxes in this field, using the deprecated line_items.taxes field results in an error. Use line_items.applied_taxes instead.
    pub taxes: Option<Vec<OrderLineItemTax>>,
    /// The list of all discounts associated with the order.
    ///
    /// Discounts can be scoped to either ORDER or LINE_ITEM. For discounts scoped to LINE_ITEM, an OrderLineItemAppliedDiscount must be added to each line item that the discount applies to. For discounts with ORDER scope, the server generates an OrderLineItemAppliedDiscount for every line item.
    ///
    /// IMPORTANT: If LINE_ITEM scope is set on any discounts in this field, using the deprecated line_items.discounts field results in an error. Use line_items.applied_discounts instead.
    pub discounts: Option<Vec<OrderLineItemDiscount>>,
    /// A list of service charges applied to the order.
    pub service_charges: Option<Vec<OrderServiceCharge>>,
    /// Details about order fulfillment.
    ///
    /// Orders can only be created with at most one fulfillment. However, orders returned by the API might contain multiple fulfillments.
    pub fulfillments: Option<Vec<Fulfillment>>,
    /// Read only A collection of items from sale orders being returned in this one. Normally part of an itemized return or exchange. There is exactly one Return object per sale Order being referenced.
    pub returns: Option<Vec<OrderReturn>>,
    /// Read only The rollup of the returned money amounts.
    pub return_amounts: Option<OrderMoneyAmounts>,
    /// Read only The net money amounts (sale money - return money).
    pub net_amounts: Option<OrderMoneyAmounts>,
    /// Read only A positive rounding adjustment to the total of the order. This adjustment is commonly used to apply cash rounding when the minimum unit of account is smaller than the lowest physical denomination of the currency.
    pub rounding_adjustment: Option<OrderRoundingAdjustment>,
    /// Read only The tenders that were used to pay for the order.
    pub tenders: Option<Vec<Tender>>,
    /// Read only The refunds that are part of this order.
    pub refunds: Option<Vec<Refund>>,
    /// Application-defined data attached to this order. Metadata fields are intended to store descriptive references or associations with an entity in another system or store brief information about the object. Square does not process this field; it only stores and returns it in relevant API calls. Do not use metadata to store any sensitive information (such as personally identifiable information or card details).
    ///
    /// Keys written by applications must be 60 characters or less and must be in the character set [a-zA-Z0-9_-]. Entries can also include metadata generated by Square. These keys are prefixed with a namespace, separated from the key with a ':' character.
    ///
    /// Values have a maximum length of 255 characters.
    ///
    /// An application can have up to 10 entries per metadata field.
    ///
    /// Entries written by applications are private and can only be read or modified by the same application.
    ///
    /// For more information, see [Metadata](https://developer.squareup.com/docs/build-basics/metadata).
    pub metadata: Option<HashMap<String, String>>,
    /// The timestamp for when the order was created, in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// The timestamp for when the order was last updated, in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
    /// The timestamp for when the order reached a terminal state, in RFC 3339 format (for example "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub closed_at: Option<String>,
    /// The current state of the order.
    pub state: Option<OrderState>,
    /// The version number, which is incremented each time an update is committed to the order. Orders not created through the API do not include a version number and therefore cannot be updated.
    ///
    /// [Read more about working with versions](https://developer.squareup.com/docs/orders-api/manage-orders/update-orders).
    pub version: Option<i32>,
    /// Read only The total amount of money to collect for the order.
    pub total_money: Option<Money>,
    /// Read only The total amount of tax money to collect for the order.
    pub total_tax_money: Option<Money>,
    /// Read only The total amount of discount money to collect for the order.
    pub total_discount_money: Option<Money>,
    /// Read only The total amount of tip money to collect for the order.
    pub total_tip_money: Option<Money>,
    /// Read only The total amount of money collected in service charges for the order.
    ///
    /// Note: total_service_charge_money is the sum of applied_money fields for each individual service charge. Therefore, total_service_charge_money only includes inclusive tax amounts, not additive tax amounts.
    pub total_service_charge_money: Option<Money>,
    /// A short-term identifier for the order (such as a customer first name, table number, or auto-generated order number that resets daily).
    ///
    /// Max Length 30
    pub ticket_name: Option<String>,
    /// Pricing options for an order. The options affect how the order's price is calculated. They can be used, for example, to apply automatic price adjustments that are based on preconfigured [pricing rules](https://developer.squareup.com/reference/square/objects/CatalogPricingRule).
    pub pricing_options: Option<OrderPricingOptions>,
    /// Read only A set-like list of Rewards that have been added to the Order.
    pub rewards: Option<Vec<OrderReward>>,
    /// Read only The net amount of money due on the order.
    pub net_amount_due_money: Option<Money>,
}
