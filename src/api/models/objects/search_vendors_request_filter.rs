//! SearchVendorsRequestFilter

use serde::{Deserialize, Serialize};

/// Defines supported query expressions to search for vendors by.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchVendorsRequestFilter {
    /// The names of the [Vendor](https://developer.squareup.com/reference/square/objects/Vendor) objects to retrieve.
    pub name: Option<Vec<String>>,
    /// The statuses of the [Vendor](https://developer.squareup.com/reference/square/objects/Vendor) objects to retrieve.
    pub status: Option<Vec<VendorStatus>>,
}
