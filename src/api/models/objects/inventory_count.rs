//! InventoryCount

use serde::{Deserialize, Serialize};

/// Represents Square-estimated quantity of items in a particular state at a particular seller location based on the known history of physical counts and inventory adjustments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryCount {
    /// The Square-generated ID of the [CatalogObject](https://developer.squareup.com/reference/square/objects/CatalogObject) being tracked.
    ///
    /// Max Length 100
    pub catalog_object_id: Option<String>,
    /// The [type](https://developer.squareup.com/reference/square/objects/CatalogObjectType) of the [CatalogObject](https://developer.squareup.com/reference/square/objects/CatalogObject) being tracked.
    ///
    /// The Inventory API supports setting and reading the "catalog_object_type": "ITEM_VARIATION" field value. In addition, it can also read the "catalog_object_type": "ITEM" field value that is set by the Square Restaurants app.
    ///
    /// Max Length 14
    pub catalog_object_type: Option<String>,
    /// The current [inventory state](https://developer.squareup.com/reference/square/objects/InventoryState) for the related quantity of items.
    pub state: Option<InventoryState>,
    /// The Square-generated ID of the [Location](https://developer.squareup.com/reference/square/objects/Location) where the related quantity of items is being tracked.
    ///
    /// Max Length 100
    pub location_id: Option<String>,
    /// The number of items affected by the estimated count as a decimal string. Can support up to 5 digits after the decimal point.
    ///
    /// Max Length 26
    pub quantity: Option<String>,
    /// Read only An RFC 3339-formatted timestamp that indicates when the most recent physical count or adjustment affecting the estimated count is received.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    ///
    /// Max Length 34
    pub calculated_at: Option<String>,
    /// Read only Whether the inventory count is for composed variation (TRUE) or not (FALSE). If true, the inventory count will not be present in the response of any of these endpoints: [BatchChangeInventory](https://developer.squareup.com/reference/square/inventory-api/batch-change-inventory), [BatchRetrieveInventoryChanges](https://developer.squareup.com/reference/square/inventory-api/batch-retrieve-inventory-changes), [BatchRetrieveInventoryCounts](https://developer.squareup.com/reference/square/inventory-api/batch-retrieve-inventory-counts), and [RetrieveInventoryChanges](https://developer.squareup.com/reference/square/inventory-api/retrieve-inventory-changes).
    pub is_estimated: Option<bool>,
}
