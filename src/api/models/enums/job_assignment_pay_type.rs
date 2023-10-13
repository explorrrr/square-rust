//! JobAssignmentPayType Enum

use serde::{Deserialize, Serialize};

/// Enumerates the possible pay types that a job can be assigned.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobAssignmentPayType {
    /// The job does not have a defined pay type.
    None,
    /// The job pays an hourly rate.
    Hourly,
    /// The job pays an annual salary.
    Salary,
}
