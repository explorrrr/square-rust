//! DeviceStatusCategory Enum

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceStatusCategoryV20230925 {
    /// The device status is available.
    Available,
    /// The device status needs attention.
    NeedsAttention,
}
