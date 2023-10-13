//! VendorContact

use serde::{Deserialize, Serialize};

/// Represents a contact of a [Vendor](https://developer.squareup.com/reference/square/objects/Vendor).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorContactV20230925 {
    /// A unique Square-generated ID for the [VendorContact](https://developer.squareup.com/reference/square/objects/VendorContact). This field is required when attempting to update a [VendorContact](https://developer.squareup.com/reference/square/objects/VendorContact).
    ///
    /// Max Length 100
    pub id: Option<String>,
    /// The name of the [VendorContact](https://developer.squareup.com/reference/square/objects/VendorContact). This field is required when attempting to create a [Vendor](https://developer.squareup.com/reference/square/objects/Vendor).
    ///
    /// Max Length 255
    pub name: Option<String>,
    /// The email address of the [VendorContact](https://developer.squareup.com/reference/square/objects/VendorContact).
    ///
    /// Max Length 255
    pub email_address: Option<String>,
    /// The phone number of the [VendorContact](https://developer.squareup.com/reference/square/objects/VendorContact).
    ///
    /// Max Length 255
    pub phone_number: Option<String>,
    /// The state of the [VendorContact](https://developer.squareup.com/reference/square/objects/VendorContact).
    pub removed: Option<bool>,
    /// The ordinal of the [VendorContact](https://developer.squareup.com/reference/square/objects/VendorContact).
    pub ordinal: i32,
}
