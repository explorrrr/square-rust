//! InventoryChangeType Enum

use serde::{Deserialize, Serialize};

/// Indicates how the inventory change was applied to a tracked product quantity.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InventoryChangeTypeV20230925 {
    /// The change occurred as part of a physical count update.
    PhysicalCount,
    /// The change occurred as part of the normal lifecycle of goods (e.g., as an inventory adjustment).
    Adjustment,
    /// The change occurred as part of an inventory transfer.
    Transfer,
}
