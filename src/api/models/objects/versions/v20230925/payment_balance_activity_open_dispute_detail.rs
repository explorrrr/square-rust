//! PaymentBalanceActivityOpenDisputeDetail

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBalanceActivityOpenDisputeDetailV20230925 {
    /// The ID of the payment associated with this activity.
    payment_id: Option<String>,
    /// The ID of the dispute associated with this activity.
    dispute_id: Option<String>,
}