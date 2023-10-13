//! CustomerCustomAttributeFilters

use serde::{Deserialize, Serialize};

use super::customer_custom_attribute_filter::CustomerCustomAttributeFilterV20230925;
/// The custom attribute filters in a set of [customer filters](https://developer.squareup.com/reference/square/objects/CustomerFilter) used in a search query.
///
/// Use this filter to search based on [custom attributes](https://developer.squareup.com/reference/square/objects/CustomAttribute) that are assigned to customer profiles. For more information, see [Search by custom attribute](https://developer.squareup.com/docs/customers-api/use-the-api/search-customers#search-by-custom-attribute).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerCustomAttributeFiltersV20230925 {
    /// The custom attribute filters. Each filter must specify `key` and include the `filter` field with a type-specific filter, the `updated_at` field, or both. The provided keys must be unique within the list of custom attribute filters.
    pub filters: Option<Vec<CustomerCustomAttributeFilterV20230925>>,
}
