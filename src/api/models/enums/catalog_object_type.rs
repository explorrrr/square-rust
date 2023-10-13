//! CatalogObjectType enum

use serde::{Deserialize, Serialize};

/// Possible types of CatalogObjects returned from the catalog, each containing type-specific properties in the *_data field corresponding to the specified object type.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogObjectType {
    /// The CatalogObject instance is of the [CatalogItem](https://developer.squareup.com/reference/square/objects/CatalogItem) type and represents an item. The item-specific data must be set on the item_data field.
    Item,
    /// The CatalogObject instance is of the [CatalogImage](https://developer.squareup.com/reference/square/objects/CatalogImage) type and represents an image. The image-specific data must be set on the image_data field.
    Image,
    /// The CatalogObject instance is of the [CatalogCategory](https://developer.squareup.com/reference/square/objects/CatalogCategory) type and represents a category. The category-specific data must be set on the category_data field.
    Category,
    /// The CatalogObject instance is of the [CatalogItemVariation](https://developer.squareup.com/reference/square/objects/CatalogItemVariation) type and represents an item variation, also referred to as variation. The item variation-specific data must be set on the item_variation_data field.
    ItemVariation,
    /// The CatalogObject instance is of the [CatalogTax](https://developer.squareup.com/reference/square/objects/CatalogTax) type and represents a tax. The tax-specific data must be set on the tax_data field.
    Tax,
    /// The CatalogObject instance is of the [CatalogDiscount](https://developer.squareup.com/reference/square/objects/CatalogDiscount) type and represents a discount. The discount-specific data must be set on the discount_data field.
    Discount,
    /// The CatalogObject instance is of the [CatalogModifierList](https://developer.squareup.com/reference/square/objects/CatalogModifierList) type and represents a modifier list. The modifier-list-specific data must be set on the modifier_list_data field.
    ModifierList,
    /// The CatalogObject instance is of the [CatalogModifier](https://developer.squareup.com/reference/square/objects/CatalogModifier) type and represents a modifier. The modifier-specific data must be set on the modifier_data field.
    Modifier,
    /// The CatalogObject instance is of the [CatalogPricingRule](https://developer.squareup.com/reference/square/objects/CatalogPricingRule) type and represents a pricing rule. The pricing-rule-specific data must be set on the pricing_rule_data field.
    PricingRule,
    /// The CatalogObject instance is of the [CatalogProductSet](https://developer.squareup.com/reference/square/objects/CatalogProductSet) type and represents a product set. The product-set-specific data will be stored in the product_set_data field.
    ProductSet,
    /// The CatalogObject instance is of the [CatalogTimePeriod](https://developer.squareup.com/reference/square/objects/CatalogTimePeriod) type and represents a time period. The time-period-specific data must be set on the time_period_data field.
    TimePeriod,
    /// The CatalogObject instance is of the [CatalogMeasurementUnit](https://developer.squareup.com/reference/square/objects/CatalogMeasurementUnit) type and represents a measurement unit specifying the unit of measure and precision in which an item variation is sold. The measurement-unit-specific data must set on the measurement_unit_data field.
    MeasurementUnit,
    /// The CatalogObject instance is of the [CatalogSubscriptionPlan](https://developer.squareup.com/reference/square/objects/CatalogSubscriptionPlan) type and represents a subscription plan. The subscription-plan-specific data must be stored on the subscription_plan_data field.
    SubscriptionPlanVariation,
    /// The CatalogObject instance is of the [CatalogItemOption](https://developer.squareup.com/reference/square/objects/CatalogItemOption) type and represents a list of options (such as a color or size of a T-shirt) that can be assigned to item variations. The item-option-specific data must be on the item_option_data field.
    ItemOption,
    /// The CatalogObject instance is of the [CatalogItemOptionValue](https://developer.squareup.com/reference/square/objects/CatalogItemOptionValue) type and represents a value associated with one or more item options. For example, an item option of "Size" may have item option values such as "Small" or "Medium". The item-option-value-specific data must be on the item_option_value_data field.
    ItemOptionVal,
    /// The CatalogObject instance is of the [CatalogCustomAttributeDefinition](https://developer.squareup.com/reference/square/objects/CatalogCustomAttributeDefinition) type and represents the definition of a custom attribute. The custom-attribute-definition-specific data must be set on the custom_attribute_definition_data field.
    CustomAttributeDefinition,
    /// The Catalog Object instance is of the [CatalogQuickAmountsSettings](https://developer.squareup.com/reference/square/objects/CatalogQuickAmountsSettings) type and represents settings to configure preset charges for quick payments at each location. For example, a location may have a list of both AUTO and MANUAL quick amounts that are set to DISABLED. The quick-amounts-settings-specific data must be set on the quick_amounts_settings_data field.
    QuickAmountsSettings,
    /// The CatalogObject instance is of the [CatalogSubscriptionPlan](https://developer.squareup.com/reference/square/objects/CatalogSubscriptionPlan) type and represents a subscription plan. The subscription-plan-specific data must be stored on the subscription_plan_data field.
    SubscriptionPlan,
}
