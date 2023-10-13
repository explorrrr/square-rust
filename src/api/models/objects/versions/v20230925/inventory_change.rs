//! InventoryChange

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::inventory_change_type::InventoryChangeTypeV20230925;

use super::{inventory_physical_count::InventoryPhysicalCountV20230925, inventory_adjustment::InventoryAdjustmentV20230925, inventory_transfer::InventoryTransferV20230925, catalog_measurement_unit::CatalogMeasurementUnitV20230925};

/// Represents a single physical count, inventory, adjustment, or transfer that is part of the history of inventory changes for a particular [CatalogObject](https://developer.squareup.com/reference/square/objects/CatalogObject) instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryChangeV20230925 {
    /// Indicates how the inventory change is applied. See [InventoryChangeType](https://developer.squareup.com/reference/square/enums/InventoryChangeType) for all possible values.
    pub r#type: Option<InventoryChangeTypeV20230925>,
    /// Contains details about the physical count when `type` is `PHYSICAL_COUNT`, and is unset for all other change types.
    pub physical_count: Option<InventoryPhysicalCountV20230925>,
    /// Contains details about the inventory adjustment when `type` is `ADJUSTMENT`, and is unset for all other change types.
    pub adjustment: Option<InventoryAdjustmentV20230925>,
    /// Contains details about the inventory transfer when `type` is `TRANSFER`, and is unset for all other change types.
    ///
    /// Note: An [InventoryTransfer](https://developer.squareup.com/reference/square/objects/InventoryTransfer) object can only be set in the input to the [BatchChangeInventory](https://developer.squareup.com/reference/square/inventory-api/batch-change-inventory) endpoint when the seller has an active Retail Plus subscription.
    pub transfer: Option<InventoryTransferV20230925>,
    /// Read only The [CatalogMeasurementUnit](https://developer.squareup.com/reference/square/objects/CatalogMeasurementUnit) object representing the catalog measurement unit associated with the inventory change.
    pub measurement_unit: Option<CatalogMeasurementUnitV20230925>,
    /// Read only The ID of the [CatalogMeasurementUnit](https://developer.squareup.com/reference/square/objects/CatalogMeasurementUnit) object representing the catalog measurement unit associated with the inventory change.
    pub measurement_unit_id: Option<String>,
}
