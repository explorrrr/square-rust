//! LoyaltyAccountExpiringPointDeadline

use serde::{Deserialize, Serialize};

/// Represents a set of points for a loyalty account that are scheduled to expire on a specific date.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyAccountExpiringPointDeadline {
    /// The number of points scheduled to expire at the expires_at timestamp.
    pub points: i32,
    /// The timestamp of when the points are scheduled to expire, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// * UTC: 2020-01-26T02:25:34Z
    ///
    /// * Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    ///
    /// Min Length 1
    pub expires_at: String,
}
