//! RiskEvaluationRiskLevel Enum

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskEvaluationRiskLevelV20230925 {
    /// Indicates Square is still evaluating the payment.
    Pending,
    /// Indicates payment risk is within the normal range.
    Normal,
    /// Indicates elevated risk level associated with the payment.
    Moderate,
    /// Indicates significantly elevated risk level with the payment.
    High,
}
