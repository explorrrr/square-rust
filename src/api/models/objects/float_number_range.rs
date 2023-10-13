//! FloatNumberRange

use serde::{Deserialize, Serialize};

/// Specifies a decimal number range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatNumberRange {
    /// A decimal value indicating where the range starts.
    pub start_at: Option<String>,
    /// A decimal value indicating where the range ends.
    pub end_at: Option<String>,
}
