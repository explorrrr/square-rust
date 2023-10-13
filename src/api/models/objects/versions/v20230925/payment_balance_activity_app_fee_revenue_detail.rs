//! PaymentBalanceActivityAppFeeRevenueDetail

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBalanceActivityAppFeeRevenueDetailV20230925 {
    /// The ID of the payment associated with this activity.
    payment_id: Option<String>,
    /// The ID of the location of the merchant associated with the payment activity
    location_id: Option<String>,
}
