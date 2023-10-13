//! CustomerCreationSourceFilter

use serde::{Deserialize, Serialize};

/// The creation source filter.
///
/// If one or more creation sources are set, customer profiles are included in, or excluded from, the result if they match at least one of the filter criteria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerCreationSourceFilter {
    /// The list of creation sources used as filtering criteria.
    pub values: Option<Vec<CustomerCreationSource>>,
    /// Indicates whether a customer profile matching the filter criteria should be included in the result or excluded from the result.
    ///
    /// Default: INCLUDE.
    pub rule: Option<CustomerInclusionExclusion>,
}
