//! CatalogCustomAttributeDefinition

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::catalog_custom_attribute_definition_app_visibility::CatalogCustomAttributeDefinitionAppVisibilityV20230925;
use crate::api::models::enums::versions::v20230925::catalog_custom_attribute_definition_seller_visibility::CatalogCustomAttributeDefinitionSellerVisibilityV20230925;
use crate::api::models::enums::versions::v20230925::catalog_custom_attribute_definition_type::CatalogCustomAttributeDefinitionTypeV20230925;
use crate::api::models::enums::versions::v20230925::catalog_object_type::CatalogObjectTypeV20230925;
use crate::api::models::objects::versions::v20230925::catalog_custom_attribute_definition_number_config::CatalogCustomAttributeDefinitionNumberConfigV20230925;
use crate::api::models::objects::versions::v20230925::catalog_custom_attribute_definition_selection_config::CatalogCustomAttributeDefinitionSelectionConfigV20230925;
use crate::api::models::objects::versions::v20230925::catalog_custom_attribute_definition_string_config::CatalogCustomAttributeDefinitionStringConfigV20230925;
use crate::api::models::objects::versions::v20230925::source_application::SourceApplicationV20230925;

/// Contains information defining a custom attribute.
///
/// Custom attributes are intended to store additional information about a catalog object or to associate a catalog object with an entity in another system. Do not use custom attributes to store any sensitive information (personally identifiable information, card details, etc.). [Read more about custom attributes](https://developer.squareup.com/docs/catalog-api/add-custom-attributes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogCustomAttributeDefinitionV20230925 {
    /// The type of this custom attribute. Cannot be modified after creation. Required.
    pub r#type: CatalogCustomAttributeDefinitionTypeV20230925,
    /// The name of this definition for API and seller-facing UI purposes. The name must be unique within the (merchant, application) pair. Required. May not be empty and may not exceed 255 characters. Can be modified after creation.
    /// Min Length 1
    /// Max Length 255
    pub name: String,
    /// Seller-oriented description of the meaning of this Custom Attribute, any constraints that the seller should observe, etc. May be displayed as a tooltip in Square UIs.
    /// Max Length 255
    pub description: Option<String>,
    /// Read only. Contains information about the application that created this custom attribute definition.
    pub source_application: Option<SourceApplicationV20230925>,
    /// The set of CatalogObject types that this custom atttribute may be applied to. Currently, only ITEM, ITEM_VARIATION, and MODIFIER are allowed. At least one type must be included.
    pub allowed_object_types: Vec<CatalogObjectTypeV20230925>,
    /// The visibility of a custom attribute in seller-facing UIs (including Square Point of Sale applications and Square Dashboard). May be modified.
    pub seller_visibility: Option<CatalogCustomAttributeDefinitionSellerVisibilityV20230925>,
    /// The visibility of a custom attribute to applications other than the application that created the attribute.
    pub app_visibility: Option<CatalogCustomAttributeDefinitionAppVisibilityV20230925>,
    /// Optionally, populated when type = STRING, unset otherwise.
    pub string_config: Option<CatalogCustomAttributeDefinitionStringConfigV20230925>,
    /// Optionally, populated when type = NUMBER, unset otherwise.
    pub number_config: Option<CatalogCustomAttributeDefinitionNumberConfigV20230925>,
    /// Populated when type is set to SELECTION, unset
    pub selection_config: Option<CatalogCustomAttributeDefinitionSelectionConfigV20230925>,
    /// Read only The number of custom attributes that reference this custom attribute definition. Set by the server in response to a ListCatalog request with include_counts set to true. If the actual count is greater than 100, custom_attribute_usage_count will be set to 100.
    pub custom_attribute_usage_count: Option<i32>,
    /// The name of the desired custom attribute key that can be used to access the custom attribute value on catalog objects. Cannot be modified after the custom attribute definition has been created. Must be between 1 and 60 characters, and may only contain the characters [a-zA-Z0-9_-].
    /// Min Length 1
    /// Max Length 60
    pub key: Option<String>,
}
