//! PhaseInput

use serde::{Deserialize, Serialize};

/// Represents the arguments used to construct a new phase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseInput {
    /// index of phase in total subscription plan
    pub ordinal: i32,
    /// id of order to be used in billing
    pub order_template_id: Option<String>,
}
