//! ProcessingFee

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;

/// Represents the Square processing fee.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingFeeV20230925 {
    /// The timestamp of when the fee takes effect, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub effective_at: Option<String>,
    /// The type of fee assessed or adjusted. The fee type can be INITIAL or ADJUSTMENT.
    pub r#type: Option<String>,
    /// The fee amount, which might be negative, that is assessed or adjusted by Square.
    ///
    /// Positive values represent funds being assessed, while negative values represent funds being returned.
    pub amount_money: Option<MoneyV20230925>,
}
