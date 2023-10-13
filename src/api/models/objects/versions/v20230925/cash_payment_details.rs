//! CashPaymentDetails

use serde::{Deserialize, Serialize};

use crate::api::models::objects::versions::v20230925::money::MoneyV20230925;

/// Stores details about a cash payment.
///
/// Contains only non-confidential information. For more information, see [Take Cash Payments](https://developer.squareup.com/docs/payments-api/take-payments/cash-payments).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashPaymentDetailsV20230925 {
    /// The amount and currency of the money supplied by the buyer.
    pub buyer_supplied_money: MoneyV20230925,
    /// The amount of change due back to the buyer. This read-only field is calculated from the amount_money and buyer_supplied_money fields.
    pub change_back_money: Option<MoneyV20230925>,
}
