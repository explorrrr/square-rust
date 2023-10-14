//! CustomerCreationSourceFilter

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::{
    customer_creation_source::CustomerCreationSourceV20230925,
    customer_inclusion_exclusion::CustomerInclusionExclusionV20230925,
};

/// The creation source filter.
///
/// If one or more creation sources are set, customer profiles are included in, or excluded from, the result if they match at least one of the filter criteria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerCreationSourceFilterV20230925 {
    /// The list of creation sources used as filtering criteria.
    pub values: Option<Vec<CustomerCreationSourceV20230925>>,
    /// Indicates whether a customer profile matching the filter criteria should be included in the result or excluded from the result.
    ///
    /// Default: INCLUDE.
    pub rule: Option<CustomerInclusionExclusionV20230925>,
}
