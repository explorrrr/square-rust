//! OrderServiceChargeType Enum

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderServiceChargeType {
    AutoGratuity,
    Custom,
}
