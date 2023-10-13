//! PayoutFee

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::payout_fee_type::PayoutFeeTypeV20230925;

use super::money::MoneyV20230925;

/// Represents a payout fee that can incur as part of a payout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutFeeV20230925 {
    /// The money amount of the payout fee.
    pub amount_money: Option<MoneyV20230925>,
    /// The timestamp of when the fee takes effect, in RFC 3339 format.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub effective_at: Option<String>,
    /// The type of fee assessed as part of the payout.
    pub r#type: Option<PayoutFeeTypeV20230925>,
}
