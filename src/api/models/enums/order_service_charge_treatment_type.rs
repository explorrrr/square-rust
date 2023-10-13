//! OrderServiceChargeTreatmentType Enum

use serde::{Deserialize, Serialize};

/// Indicates whether the service charge will be treated as a value-holding line item or apportioned toward a line item.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderServiceChargeTreatmentType {
    LineItemTreatment,
    ApportionedTreatment,
}
