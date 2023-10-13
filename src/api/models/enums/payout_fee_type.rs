//! PayoutFeeType Enum

use serde::{Deserialize, Serialize};

/// Represents the type of payout fee that can incur as part of a payout.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayoutFeeType {
    /// Fee type associated with transfers.
    TransferFee,
    /// Taxes associated with the transfer fee.
    TaxOnTransferFee,
}
