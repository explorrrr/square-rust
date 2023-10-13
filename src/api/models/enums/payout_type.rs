//! PayoutType Enum

use serde::{Deserialize, Serialize};

/// The type of payout: “BATCH” or “SIMPLE”.
/// BATCH payouts include a list of payout entries that can be considered settled. SIMPLE payouts do not have any payout entries associated with them and will show up as one of the payout entries in a future BATCH payout.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayoutType {
    /// Payouts that include a list of payout entries that can be considered settled.
    Batch,
    /// Payouts that do not have any payout entries associated with them and will show up as one of the payout entries in a future BATCH payout.
    Simple,
}
