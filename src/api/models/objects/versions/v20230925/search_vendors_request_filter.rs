//! SearchVendorsRequestFilter

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::vendor_status::VendorStatusV20230925;

/// Defines supported query expressions to search for vendors by.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchVendorsRequestFilterV20230925 {
    /// The names of the [Vendor](https://developer.squareup.com/reference/square/objects/Vendor) objects to retrieve.
    pub name: Option<Vec<String>>,
    /// The statuses of the [Vendor](https://developer.squareup.com/reference/square/objects/Vendor) objects to retrieve.
    pub status: Option<Vec<VendorStatusV20230925>>,
}
