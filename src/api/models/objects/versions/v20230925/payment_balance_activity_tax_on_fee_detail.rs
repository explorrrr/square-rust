//! PaymentBalanceActivityTaxOnFeeDetail

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBalanceActivityTaxOnFeeDetailV20230925 {
    /// The ID of the payment associated with this activity.
    payment_id: Option<String>,
    /// The description of the tax rate being applied. For example: "GST", "HST".
    tax_rate_description: Option<String>,
}
