//! Range

use serde::{Deserialize, Serialize};

/// The range of a number value between the specified lower and upper bounds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    /// The lower bound of the number range. At least one of min or max must be specified. If unspecified, the results will have no minimum value.
    min: Option<String>,
    /// The upper bound of the number range. At least one of min or max must be specified. If unspecified, the results will have no maximum value.
    max: Option<String>,
}
