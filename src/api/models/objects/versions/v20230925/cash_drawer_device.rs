//! CashDrawerDevice

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashDrawerDeviceV20230925 {
    /// The device Square-issued ID
    pub id: Option<String>,
    /// The device merchant-specified name.
    pub name: Option<String>,
}
