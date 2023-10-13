//! Money

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::currency::CurrencyV20230925;

/// Represents an amount of money.
/// Money fields can be signed or unsigned. Fields that do not explicitly define whether they are signed or unsigned are considered unsigned and can only hold positive amounts.
/// For signed fields, the sign of the value indicates the purpose of the money transfer. See [Working with Monetary Amounts](https://developer.squareup.com/docs/build-basics/working-with-monetary-amounts) for more information.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoneyV20230925 {
    /// The amount of money, in the smallest denomination of the currency indicated by currency.
    /// For example, when currency is USD, amount is in cents. Monetary amounts can be positive or negative.
    /// See the specific field description to determine the meaning of the sign in a particular case.
    pub amount: i64,
    /// The type of currency, in __ISO 4217 format__. For example, the currency code for US dollars is USD.
    pub currency: CurrencyV20230925,
}
