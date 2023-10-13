//! PaymentBalanceActivityAutomaticSavingsReversedDetail

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBalanceActivityAutomaticSavingsReversedDetail {
    /// The ID of the payment associated with this activity.
    payment_id: Option<String>,
    /// The ID of the payout associated with this activity.
    payout_id: Option<String>,
}