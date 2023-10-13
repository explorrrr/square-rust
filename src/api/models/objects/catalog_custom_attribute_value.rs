//! CatalogCustomAttributeValue

use serde::{Deserialize, Serialize};

/// An instance of a custom attribute.
///
/// Custom attributes can be defined and added to ITEM and ITEM_VARIATION type catalog objects. Read more about custom attributes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogCustomAttributeValue {
    /// The name of the custom attribute.
    pub name: Option<String>,
    /// The string value of the custom attribute. Populated if type = STRING.
    pub string_value: Option<String>,
    /// Read only The id of the CatalogCustomAttributeDefinition this value belongs to.
    pub custom_attribute_definition_id: Option<String>,
    /// Read only A copy of type from the associated CatalogCustomAttributeDefinition.
    pub r#type: Option<CatalogCustomAttributeDefinitionType>,
    /// Populated if type = NUMBER. Contains a string representation of a decimal number, using a . as the decimal separator.
    pub number_value: Option<String>,
    /// A true or false value. Populated if type = BOOLEAN.
    pub boolean_value: Option<bool>,
    /// One or more choices from allowed_selections. Populated if type = SELECTION.
    pub selection_uid_values: Option<Vec<String>>,
    /// Read only If the associated CatalogCustomAttributeDefinition object is defined by another application, this key is prefixed by the defining application ID. For example, if the CatalogCustomAttributeDefinition has a key attribute of "cocoa_brand" and the defining application ID is "abcd1234", this key is "abcd1234:cocoa_brand" when the application making the request is different from the application defining the custom attribute definition. Otherwise, the key is simply "cocoa_brand".
    pub key: Option<String>,
}
