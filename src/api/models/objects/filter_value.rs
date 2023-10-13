//! FilterValue

use serde::{Deserialize, Serialize};

/// A filter to select resources based on an exact field value.
///
/// For any given value, the value can only be in one property. Depending on the field, either all properties can be set or only a subset will be available.
///
/// Refer to the documentation of the field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterValue {
    /// A list of terms that must be present on the field of the resource.
    pub all: Option<Vec<String>>,
    /// A list of terms where at least one of them must be present on the field of the resource.
    pub any: Option<Vec<String>>,
    /// A list of terms that must not be present on the field the resource
    pub none: Option<Vec<String>>,
}
