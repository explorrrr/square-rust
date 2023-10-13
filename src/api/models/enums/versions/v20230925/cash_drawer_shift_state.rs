//! CashDrawerShiftState

use serde::{Deserialize, Serialize};

/// The types of events on a cash drawer shift.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CashDrawerShiftStateV20230925 {
    /// An open cash drawer shift.
    Open,
    /// A cash drawer shift that is ended but has not yet had an employee content audit.
    Ended,
    /// An ended cash drawer shift that is closed with a completed employee content audit and recorded result.
    Closed,
}
