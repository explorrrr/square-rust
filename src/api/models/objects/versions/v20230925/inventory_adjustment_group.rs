//! InventoryAdjustmentGroup

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::inventory_state::InventoryStateV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryAdjustmentGroupV20230925 {
    /// A unique ID generated by Square for the InventoryAdjustmentGroup.
    ///
    /// Max Length 100
    pub id: Option<String>,
    /// The inventory adjustment of the composed variation.
    ///
    /// Max Length 100
    pub root_adjustment_id: Option<String>,
    /// Representative from_state for adjustments within the group. For example, for a group adjustment from IN_STOCK to SOLD, there can be two component adjustments in the group: one from IN_STOCKto COMPOSED and the other one from COMPOSED to SOLD. Here, the representative from_state for the InventoryAdjustmentGroup is IN_STOCK.
    pub from_state: Option<InventoryStateV20230925>,
    /// Representative to_state for adjustments within group. For example, for a group adjustment from IN_STOCK to SOLD, the two component adjustments in the group can be from IN_STOCK to COMPOSED and from COMPOSED to SOLD. Here, the representative to_state of the InventoryAdjustmentGroup is SOLD.
    pub to_state: Option<InventoryStateV20230925>,
}