//! Vendor

use serde::{Deserialize, Serialize};

use super::{address::AddressV20230925, vendor_contact::VendorContactV20230925};
use crate::api::models::enums::versions::v20230925::vendor_status::VendorStatusV20230925;

/// Represents a supplier to a seller.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorV20230925 {
    /// A unique Square-generated ID for the [Vendor](https://developer.squareup.com/reference/square/objects/Vendor). This field is required when attempting to update a [Vendor](https://developer.squareup.com/reference/square/objects/Vendor).
    ///
    /// Max Length 100
    pub id: Option<String>,
    /// Read only An RFC 3339-formatted timestamp that indicates when the [Vendor](https://developer.squareup.com/reference/square/objects/Vendor) was created.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    ///
    /// Max Length 34
    pub created_at: Option<String>,
    /// Read only An RFC 3339-formatted timestamp that indicates when the [Vendor](https://developer.squareup.com/reference/square/objects/Vendor) was last updated.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    ///
    /// Max Length 34
    pub updated_at: Option<String>,
    /// The name of the [Vendor](https://developer.squareup.com/reference/square/objects/Vendor). This field is required when attempting to create or update a [Vendor](https://developer.squareup.com/reference/square/objects/Vendor).
    ///
    /// Max Length 100
    pub name: Option<String>,
    /// The address of the [Vendor](https://developer.squareup.com/reference/square/objects/Vendor).
    pub address: Option<AddressV20230925>,
    /// The contacts of the [Vendor](https://developer.squareup.com/reference/square/objects/Vendor).
    pub contacts: Option<Vec<VendorContactV20230925>>,
    /// The account number of the [Vendor](https://developer.squareup.com/reference/square/objects/Vendor).
    ///
    /// Max Length 100
    pub account_number: Option<String>,
    /// A note detailing information about the [Vendor](https://developer.squareup.com/reference/square/objects/Vendor).
    ///
    /// Max Length 4096
    pub note: Option<String>,
    /// The version of the [Vendor](https://developer.squareup.com/reference/square/objects/Vendor).
    pub version: Option<i32>,
    /// The status of the [Vendor](https://developer.squareup.com/reference/square/objects/Vendor).
    pub status: Option<VendorStatusV20230925>,
}
