//! PaymentBalanceActivityFeeDetail

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBalanceActivityFeeDetail {
    /// The ID of the payment associated with this activity.
    payment_id: Option<String>,
}
