//! ComponentComponentType enum

use serde::{Deserialize, Serialize};

/// An enum for ComponentType.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComponentComponentTypeV20230925 {
    Application,
    CardReader,
    Battery,
    Wifi,
    Ethernet,
    Printer,
}
