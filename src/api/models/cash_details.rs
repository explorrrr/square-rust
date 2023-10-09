//! CashDetails

use crate::api::models::money::Money;

/// Represents the details of a cash payment.
pub struct CashDetails {
    /// The amount and currency of the money supplied by the buyer.
    pub buyer_supplied_money: Money,
    /// The amount of change due back to the buyer. This read-only field is calculated from the amount_money and buyer_supplied_money fields.
    pub change_back_money: Option<Money>,
}
