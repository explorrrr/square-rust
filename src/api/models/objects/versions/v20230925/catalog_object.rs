//! CatalogObject

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::catalog_object_type::CatalogObjectTypeV20230925;

use crate::api::models::objects::versions::v20230925::{catalog_custom_attribute_value::CatalogCustomAttributeValueV20230925, catalog_v1_id::CatalogV1IdV20230925, catalog_item::CatalogItemV20230925, catalog_category::CatalogCategoryV20230925, catalog_item_variation::CatalogItemVariationV20230925, catalog_discount::CatalogDiscountV20230925, catalog_modifier_list::CatalogModifierListV20230925, catalog_modifier::CatalogModifierV20230925, catalog_time_period::CatalogTimePeriodV20230925, catalog_product_set::CatalogProductSetV20230925, catalog_pricing_rule::CatalogPricingRuleV20230925, catalog_image::CatalogImageV20230925, catalog_measurement_unit::CatalogMeasurementUnitV20230925, catalog_subscription_plan::CatalogSubscriptionPlanV20230925, catalog_item_option::CatalogItemOptionV20230925, catalog_item_option_value::CatalogItemOptionValueV20230925, catalog_custom_attribute_definition::CatalogCustomAttributeDefinitionV20230925, catalog_quick_amounts_settings::CatalogQuickAmountsSettingsV20230925, catalog_subscription_plan_variation::CatalogSubscriptionPlanVariationV20230925};

use crate::api::models::objects::versions::v20230925::catalog_tax::CatalogTaxV20230925;

/// The wrapper object for the catalog entries of a given object type.
///
/// Depending on the type attribute value, a CatalogObject instance assumes a type-specific data to yield the corresponding type of catalog object.
///
/// For example, if type=ITEM, the CatalogObject instance must have the ITEM-specific data set on the item_data attribute. The resulting CatalogObject instance is also a CatalogItem instance.
///
/// In general, if type=<OBJECT_TYPE>, the CatalogObject instance must have the <OBJECT_TYPE>-specific data set on the <object_type>_data attribute. The resulting CatalogObject instance is also a Catalog<ObjectType> instance.
///
/// For more detailed discussion of the Catalog data model, please see the [Design a Catalog](https://developer.squareup.com/docs/catalog-api/design-a-catalog) guide.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogObjectV20230925 {
    /// The type of this object. Each object type has expected properties expressed in a structured format within its corresponding *_data field below.
    r#type: CatalogObjectTypeV20230925,
    /// An identifier to reference this object in the catalog. When a new CatalogObject is inserted, the client should set the id to a temporary identifier starting with a "#" character. Other objects being inserted or updated within the same request may use this identifier to refer to the new object.
    ///
    /// When the server receives the new object, it will supply a unique identifier that replaces the temporary identifier for all future references.
    ///
    /// Min Length 1
    id: String,
    /// Read only Last modification timestamp in RFC 3339 format, e.g., "2016-08-15T23:59:33.123Z" would indicate the UTC time (denoted by Z) of August 15, 2016 at 23:59:33 and 123 milliseconds.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// * UTC: 2020-01-26T02:25:34Z
    /// * Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    updated_at: Option<String>,
    /// The version of the object. When updating an object, the version supplied must match the version in the database, otherwise the write will be rejected as conflicting.
    version: Option<i64>,
    /// If true, the object has been deleted from the database. Must be false for new objects being inserted. When deleted, the updated_at field will equal the deletion time.
    is_deleted: Option<bool>,
    /// A map (key-value pairs) of application-defined custom attribute values. The value of a key-value pair is a [CatalogCustomAttributeValue](https://developer.squareup.com/reference/square/objects/CatalogCustomAttributeValue) object. The key is the key attribute value defined in the associated [CatalogCustomAttributeDefinition](https://developer.squareup.com/reference/square/objects/CatalogCustomAttributeDefinition) object defined by the application making the request.
    /// If the CatalogCustomAttributeDefinition object is defined by another application, the CatalogCustomAttributeDefinition's key attribute value is prefixed by the defining application ID. For example, if the CatalogCustomAttributeDefinition has a key attribute of "cocoa_brand" and the defining application ID is "abcd1234", the key in the map is "abcd1234:cocoa_brand" if the application making the request is different from the application defining the custom attribute definition. Otherwise, the key used in the map is simply "cocoa_brand".
    /// Application-defined custom attributes are set at a global (location-independent) level. Custom attribute values are intended to store additional information about a catalog object or associations with an entity in another system. Do not use custom attributes to store any sensitive information (personally identifiable information, card details, etc.).
    custom_attribute_values: Option<HashMap<String, CatalogCustomAttributeValueV20230925>>,
    /// The Connect v1 IDs for this object at each location where it is present, where they differ from the object's Connect V2 ID. The field will only be present for objects that have been created or modified by legacy APIs.
    catalog_v1_ids: Option<Vec<CatalogV1IdV20230925>>,
    /// If true, this object is present at all locations (including future locations), except where specified in the absent_at_location_ids field. If false, this object is not present at any locations (including future locations), except where specified in the present_at_location_ids field. If not specified, defaults to true.
    present_at_all_locations: Option<bool>,
    /// A list of locations where the object is present, even if present_at_all_locations is false. This can include locations that are deactivated.
    present_at_location_ids: Option<Vec<String>>,
    /// A list of locations where the object is not present, even if present_at_all_locations is true. This can include locations that are deactivated.
    absent_at_location_ids: Option<Vec<String>>,
    /// Structured data for a CatalogItem, set for CatalogObjects of type ITEM.
    item_data: Option<CatalogItemV20230925>,
    /// Structured data for a CatalogCategory, set for CatalogObjects of type CATEGORY.
    category_data: Option<CatalogCategoryV20230925>,
    /// Structured data for a CatalogItemVariation, set for CatalogObjects of type ITEM_VARIATION.
    item_variation_data: Option<CatalogItemVariationV20230925>,
    /// Structured data for a CatalogTax, set for CatalogObjects of type TAX.
    tax_data: Option<CatalogTaxV20230925>,
    /// Structured data for a CatalogDiscount, set for CatalogObjects of type DISCOUNT.
    discount_data: Option<CatalogDiscountV20230925>,
    /// Structured data for a CatalogModifierList, set for CatalogObjects of type MODIFIER_LIST.
    modifier_list_data: Option<CatalogModifierListV20230925>,
    /// Structured data for a CatalogModifier, set for CatalogObjects of type MODIFIER.
    modifier_data: Option<CatalogModifierV20230925>,
    /// Structured data for a CatalogTimePeriod, set for CatalogObjects of type TIME_PERIOD.
    time_period_data: Option<CatalogTimePeriodV20230925>,
    /// Structured data for a CatalogProductSet, set for CatalogObjects of type PRODUCT_SET.
    product_set_data: Option<CatalogProductSetV20230925>,
    /// Structured data for a CatalogPricingRule, set for CatalogObjects of type PRICING_RULE. A CatalogPricingRule object often works with a CatalogProductSet object or a CatalogTimePeriod object.
    pricing_rule_data: Option<CatalogPricingRuleV20230925>,
    /// Structured data for a CatalogImage, set for CatalogObjects of type IMAGE.
    image_data: Option<CatalogImageV20230925>,
    /// Structured data for a CatalogMeasurementUnit, set for CatalogObjects of type MEASUREMENT_UNIT.
    measurement_unit_data: Option<CatalogMeasurementUnitV20230925>,
    /// Structured data for a CatalogSubscriptionPlan, set for CatalogObjects of type SUBSCRIPTION_PLAN.
    subscription_plan_data: Option<CatalogSubscriptionPlanV20230925>,
    /// Structured data for a CatalogItemOption, set for CatalogObjects of type ITEM_OPTION.
    item_option_data: Option<CatalogItemOptionV20230925>,
    /// Structured data for a CatalogItemOptionValue, set for CatalogObjects of type ITEM_OPTION_VAL.
    item_option_value_data: Option<CatalogItemOptionValueV20230925>,
    /// Structured data for a CatalogCustomAttributeDefinition, set for CatalogObjects of type CUSTOM_ATTRIBUTE_DEFINITION.
    custom_attribute_definition_data: Option<CatalogCustomAttributeDefinitionV20230925>,
    /// Structured data for a CatalogQuickAmountsSettings, set for CatalogObjects of type QUICK_AMOUNTS_SETTINGS.
    quick_amounts_settings_data: Option<CatalogQuickAmountsSettingsV20230925>,
    /// Structured data for a CatalogSubscriptionPlanVariation, set for CatalogObjects of type SUBSCRIPTION_PLAN_VARIATION.
    subscription_plan_variation_data: Option<CatalogSubscriptionPlanVariationV20230925>,
}
