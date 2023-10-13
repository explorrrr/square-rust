//! VendorStatus Enum

use serde::{Deserialize, Serialize};

/// The status of the Vendor, whether a Vendor is active or inactive.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VendorStatusBeta {
    /// Vendor is active and can receive purchase orders.
    Active,
    /// Vendor is inactive and cannot receive purchase orders.
    Inactive,
}
