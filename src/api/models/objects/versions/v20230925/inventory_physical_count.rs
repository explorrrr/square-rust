//! InventoryPhysicalCount

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::inventory_state::InventoryStateV20230925;

use super::source_application::SourceApplicationV20230925;

/// Represents the quantity of an item variation that is physically present at a specific location, verified by a seller or a seller's employee.
///
/// For example, a physical count might come from an employee counting the item variations on hand or from syncing with an external system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryPhysicalCountV20230925 {
    /// A unique Square-generated ID for the [InventoryPhysicalCount](https://developer.squareup.com/reference/square/objects/InventoryPhysicalCount).
    ///
    /// Max Length 100
    pub id: Option<String>,
    /// An optional ID provided by the application to tie the [InventoryPhysicalCount](https://developer.squareup.com/reference/square/objects/InventoryPhysicalCount) to an external system.
    ///
    /// Max Length 255
    pub reference_id: Option<String>,
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
    pub state: Option<InventoryStateV20230925>,
    /// The Square-generated ID of the [Location](https://developer.squareup.com/reference/square/objects/Location) where the related quantity of items is being tracked.
    ///
    /// Max Length 100
    pub location_id: Option<String>,
    /// The number of items affected by the physical count as a decimal string. The number can support up to 5 digits after the decimal point.
    ///
    /// Max Length 26
    pub quantity: Option<String>,
    /// Read only Information about the application with which the physical count is submitted.
    pub source: Option<SourceApplicationV20230925>,
    /// The Square-generated ID of the [Employee](https://developer.squareup.com/reference/square/objects/Employee) responsible for the physical count.
    ///
    /// Max Length 100
    pub employee_id: Option<String>,
    /// The Square-generated ID of the [TeamMember](https://developer.squareup.com/reference/square/objects/TeamMember) responsible for the physical count.
    ///
    /// Max Length 100
    pub team_member_id: Option<String>,
    /// A client-generated RFC 3339-formatted timestamp that indicates when the physical count was examined. For physical count updates, the occurred_at timestamp cannot be older than 24 hours or in the future relative to the time of the request.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    ///
    /// Max Length 34
    pub occurred_at: Option<String>,
    /// Read only An RFC 3339-formatted timestamp that indicates when the physical count is received.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    ///
    /// Max Length 34
    pub created_at: Option<String>,
}
