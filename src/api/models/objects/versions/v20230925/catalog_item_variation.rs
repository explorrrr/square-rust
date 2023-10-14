//! CatalogItemVariation

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::catalog_pricing_type::CatalogPricingTypeV20230925;
use crate::api::models::enums::versions::v20230925::inventory_alert_type::InventoryAlertTypeV20230925;
use crate::api::models::objects::versions::v20230925::catalog_item_option_value_for_item_variation::CatalogItemOptionValueForItemVariationV20230925;
use crate::api::models::objects::versions::v20230925::catalog_stock_conversion::CatalogStockConversionV20230925;
use crate::api::models::objects::versions::v20230925::item_variation_location_overrides::ItemVariationLocationOverridesV20230925;
use crate::api::models::objects::versions::v20230925::money::MoneyV20230925;

/// An item variation, representing a product for sale, in the Catalog object model.
///
/// Earh [item](https://developer.squareup.com/reference/square/objects/CatalogItem) must have at least one item variation and can have at most 250 item variations.
/// An item variation can be sellable, stockable, or both if it has a unit of measure for its count for the sold number of the variation,
/// the stocked number of the variation, or both. For example, when a variation representing wine is stocked and sold by the bottle,
/// the variation is both stockable and sellable. But when a variation of the wine is sold by the glass, the sold units cannot be used as a measure of the stocked units.
/// This by-the-glass variation is sellable, but not stockable. To accurately keep track of the wine's inventory count at any time,
/// the sellable count must be converted to stockable count. Typically, the seller defines this unit conversion.
/// For example, 1 bottle equals 5 glasses. The Square API exposes the stockable_conversion property on the variation to specify the conversion.
/// Thus, when two glasses of the wine are sold, the sellable count decreases by 2, and the stockable count automatically decreases by 0.4 bottle according to the conversion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogItemVariationV20230925 {
    /// The ID of the CatalogItem associated with this item variation.
    pub item_id: Option<String>,
    /// The item variation's name. This is a searchable attribute for use in applicable query filters, and its value length is of Unicode code points.
    /// Max Length 255
    pub name: Option<String>,
    /// The item variation's SKU, if any. This is a searchable attribute for use in applicable query filters.
    pub sku: Option<String>,
    /// The universal product code (UPC) of the item variation, if any. This is a searchable attribute for use in applicable query filters.
    /// The value of this attribute should be a number of 12-14 digits long. This restriction is enforced on the Square Seller Dashboard,
    /// Square Point of Sale or Retail Point of Sale apps, where this attribute shows in the GTIN field. If a non-compliant UPC value is assigned to this attribute using the API,
    /// the value is not editable on the Seller Dashboard, Square Point of Sale or Retail Point of Sale apps unless it is updated to fit the expected format.
    pub upc: Option<String>,
    /// Read only The order in which this item variation should be displayed. This value is read-only.
    /// On writes, the ordinal for each item variation within a parent CatalogItem is set according to the item variations's position.
    /// On reads, the value is not guaranteed to be sequential or unique.
    pub ordinal: Option<i64>,
    /// Indicates whether the item variation's price is fixed or determined at the time of sale.
    pub pricing_type: Option<CatalogPricingTypeV20230925>,
    /// The item variation's price, if fixed pricing is used.
    pub price_money: Option<MoneyV20230925>,
    /// Per-location price and inventory overrides.
    pub location_overrides: Option<Vec<ItemVariationLocationOverridesV20230925>>,
    /// If true, inventory tracking is active for the variation.
    pub track_inventory: Option<bool>,
    /// Indicates whether the item variation displays an alert when its inventory quantity is less than or equal to its inventory_alert_threshold.
    pub inventory_alert_type: Option<InventoryAlertTypeV20230925>,
    /// If the inventory quantity for the variation is less than or equal to this value and inventory_alert_type is LOW_QUANTITY,
    /// the variation displays an alert in the merchant dashboard.
    /// This value is always an integer.
    pub inventory_alert_threshold: Option<i64>,
    /// Arbitrary user metadata to associate with the item variation. This attribute value length is of Unicode code points.
    /// Max Length 255
    pub user_data: Option<String>,
    /// If the CatalogItem that owns this item variation is of type APPOINTMENTS_SERVICE, then this is the duration of the service in milliseconds.
    /// For example, a 30 minute appointment would have the value 1800000, which is equal to 30 (minutes) * 60 (seconds per minute) * 1000 (milliseconds per second).
    pub service_duration: Option<i64>,
    /// If the CatalogItem that owns this item variation is of type APPOINTMENTS_SERVICE, a bool representing whether this service is available for booking.
    pub available_for_booking: Option<bool>,
    /// List of item option values associated with this item variation. Listed in the same order as the item options of the parent item.
    pub item_option_values: Option<Vec<CatalogItemOptionValueForItemVariationV20230925>>,
    /// ID of the ‘CatalogMeasurementUnit’ that is used to measure the quantity sold of this item variation. If left unset, the item will be sold in whole quantities.
    pub measurement_unit_id: Option<String>,
    /// Whether this variation can be sold. The inventory count of a sellable variation indicates the number of units available for sale.
    /// When a variation is both stockable and sellable, its sellable inventory count can be smaller than or equal to its stockable count.
    pub sellable: Option<bool>,
    /// Whether stock is counted directly on this variation (TRUE) or only on its components (FALSE).
    /// When a variation is both stockable and sellable, the inventory count of a stockable variation keeps track of the number of units of this variation in stock and is not an indicator of the number of units of the variation that can be sold.
    pub stockable: Option<bool>,
    /// The IDs of images associated with this CatalogItemVariation instance. These images will be shown to customers in Square Online Store.
    pub image_ids: Option<Vec<String>>,
    /// Tokens of employees that can perform the service represented by this variation. Only valid for variations of type APPOINTMENTS_SERVICE.
    pub team_member_ids: Option<Vec<String>>,
    /// The unit conversion rule, as prescribed by the [CatalogStockConversion](https://developer.squareup.com/reference/square/objects/CatalogStockConversion) type, that describes how this non-stockable (i.e., sellable/receivable) item variation is converted to/from the stockable item variation sharing the same parent item.
    /// With the stock conversion, you can accurately track inventory when an item variation is sold in one unit, but stocked in another unit.
    pub stockable_conversion: Option<CatalogStockConversionV20230925>,
}
