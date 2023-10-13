//! PaymentBalanceActivityRefundDetail

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBalanceActivityRefundDetail {
    /// The ID of the payment associated with this activity.
    payment_id: Option<String>,
    /// The ID of the refund associated with this activity.
    refund_id: Option<String>,
}
