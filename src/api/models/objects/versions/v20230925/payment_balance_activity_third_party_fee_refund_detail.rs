//! PaymentBalanceActivityThirdPartyFeeRefundDetail

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBalanceActivityThirdPartyFeeRefundDetailV20230925 {
    /// The ID of the payment associated with this activity.
    payment_id: Option<String>,
    /// The public refund id associated with this activity.
    refund_id: Option<String>,
}
