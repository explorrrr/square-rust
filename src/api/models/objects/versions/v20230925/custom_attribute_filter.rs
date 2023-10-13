//! CustomAttributeFilter

use serde::{Deserialize, Serialize};

use super::range::RangeV20230925;

/// Supported custom attribute query expressions for calling the [SearchCatalogItems](https://developer.squareup.com/reference/square/catalog-api/search-catalog-items) endpoint to search for items or item variations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAttributeFilterV20230925 {
    /// A query expression to filter items or item variations by matching their custom attributes' custom_attribute_definition_id property value against the the specified id. Exactly one of custom_attribute_definition_id or key must be specified.
    pub custom_attribute_definition_id: Option<String>,
    /// A query expression to filter items or item variations by matching their custom attributes' key property value against the specified key. Exactly one of custom_attribute_definition_id or key must be specified.
    pub key: Option<String>,
    /// A query expression to filter items or item variations by matching their custom attributes' string_value property value against the specified text. Exactly one of string_filter, number_filter, selection_uids_filter, or bool_filter must be specified.
    pub string_filter: Option<String>,
    /// A query expression to filter items or item variations with their custom attributes containing a number value within the specified range. Exactly one of string_filter, number_filter, selection_uids_filter, or bool_filter must be specified.
    pub number_filter: Option<RangeV20230925>,
    /// A query expression to filter items or item variations by matching their custom attributes' selection_uid_values values against the specified selection uids. Exactly one of string_filter, number_filter, selection_uids_filter, or bool_filter must be specified.
    pub selection_uids_filter: Option<Vec<String>>,
    /// A query expression to filter items or item variations by matching their custom attributes' boolean_value property values against the specified Boolean expression. Exactly one of string_filter, number_filter, selection_uids_filter, or bool_filter must be specified.
    pub bool_filter: Option<bool>,
}
