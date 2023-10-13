//! LoyaltyEventAdjustPoints

use serde::{Deserialize, Serialize};

/// Provides metadata when the event type is ADJUST_POINTS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventAdjustPoints {
    /// The Square-assigned ID of the [loyalty program](https://developer.squareup.com/reference/square/objects/LoyaltyProgram).
    ///
    /// Max Length 36
    pub loyalty_program_id: Option<String>,
    /// The number of points added or removed.
    ///
    /// Min 1
    pub points: i32,
    /// The reason for the adjustment of points.
    pub reason: Option<String>,
}
