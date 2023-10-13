//! CustomerAddressFilter

use serde::{Deserialize, Serialize};

/// The customer address filter.
///
/// This filter is used in a CustomerCustomAttributeFilterValue filter when searching by an Address-type custom attribute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAddressFilter {
    /// The postal code to search for. Only an exact match is supported.
    pub postal_code: Option<CustomerTextFilter>,
    /// The country code to search for.
    pub country: Option<Country>,
}
