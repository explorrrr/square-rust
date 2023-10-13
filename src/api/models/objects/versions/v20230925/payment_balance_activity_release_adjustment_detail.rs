//! PaymentBalanceActivityReleaseAdjustmentDetail

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBalanceActivityReleaseAdjustmentDetailV20230925 {
    /// The ID of the payment associated with this activity.
    payment_id: Option<String>,
}
