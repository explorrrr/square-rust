//! PaymentBalanceActivityThirdPartyFeeRefundDetail

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBalanceActivityThirdPartyFeeRefundDetail {
    /// The ID of the payment associated with this activity.
    payment_id: Option<String>,
    /// The public refund id associated with this activity.
    refund_id: Option<String>,
}
