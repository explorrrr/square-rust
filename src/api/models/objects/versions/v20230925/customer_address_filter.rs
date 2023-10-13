//! CustomerAddressFilter

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::country::CountryV20230925;

use super::customer_text_filter::CustomerTextFilterV20230925;

/// The customer address filter.
///
/// This filter is used in a CustomerCustomAttributeFilterValue filter when searching by an Address-type custom attribute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAddressFilterV20230925 {
    /// The postal code to search for. Only an exact match is supported.
    pub postal_code: Option<CustomerTextFilterV20230925>,
    /// The country code to search for.
    pub country: Option<CountryV20230925>,
}
