//! RiskEvaluation

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::risk_evaluation_risk_level::RiskEvaluationRiskLevelV20230925;

/// Represents fraud risk information for the associated payment.
///
/// When you take a payment through Square's Payments API (using the CreatePayment endpoint), Square evaluates it and assigns a risk level to the payment. Sellers can use this information to determine the course of action (for example, provide the goods/services or refund the payment).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskEvaluationV20230925 {
    /// The timestamp when payment risk was evaluated, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    created_at: Option<String>,
    /// The risk level associated with the payment.
    risk_level: Option<RiskEvaluationRiskLevelV20230925>,
}
