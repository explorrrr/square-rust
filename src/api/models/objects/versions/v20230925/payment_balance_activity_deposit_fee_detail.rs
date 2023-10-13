//! PaymentBalanceActivityDepositFeeDetail

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBalanceActivityDepositFeeDetailV20230925 {
    /// The ID of the payout that triggered this deposit fee activity.
    payout_id: Option<String>,
}
