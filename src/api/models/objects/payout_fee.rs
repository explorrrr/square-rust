//! PayoutFee

use serde::{Deserialize, Serialize};

/// Represents a payout fee that can incur as part of a payout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutFee {
    /// The money amount of the payout fee.
    pub amount_money: Option<Money>,
    /// The timestamp of when the fee takes effect, in RFC 3339 format.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub effective_at: Option<String>,
    /// The type of fee assessed as part of the payout.
    pub r#type: Option<PayoutFeeType>,
}
