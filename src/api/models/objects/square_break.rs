//! SquareBreak

use serde::{Deserialize, Serialize};

/// A record of an employee's break during a shift.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SquareBreak {
    /// The UUID for this object.
    pub id: Option<String>,
    /// RFC 3339; follows the same timezone information as Shift. Precision up to the minute is respected; seconds are truncated.
    ///
    /// Min Length 1
    pub start_at: String,
    /// RFC 3339; follows the same timezone information as Shift. Precision up to the minute is respected; seconds are truncated.
    pub end_at: Option<String>,
    /// The BreakType that this Break was templated on.
    ///
    /// Min Length 1
    pub break_type_id: String,
    /// A human-readable name.
    ///
    /// Min Length 1
    pub name: String,
    /// Format: RFC-3339 P[n]Y[n]M[n]DT[n]H[n]M[n]S. The expected length of the break.
    ///
    /// Min Length 1
    pub expected_duration: String,
    /// Whether this break counts towards time worked for compensation purposes.
    pub is_paid: bool,
}
