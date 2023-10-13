//! DeviceCode

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::{product_type::ProductTypeV20230925, device_code_status::DeviceCodeStatusV20230925};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCodeV20230925 {
    /// Read only The unique id for this device code.
    pub id: Option<String>,
    /// An optional user-defined name for the device code.
    ///
    /// Max Length: 128
    pub name: Option<String>,
    /// Read only The unique code that can be used to login.
    pub code: Option<String>,
    /// Read only The unique id of the device that used this code. Populated when the device is paired up.
    pub device_id: Option<String>,
    /// The targeting product type of the device code.
    pub product_type: ProductTypeV20230925,
    /// The location assigned to this code.
    /// Max Length: 50
    pub location_id: Option<String>,
    /// Read only The pairing status of the device code.
    pub status: DeviceCodeStatusV20230925,
    /// Read only When this DeviceCode will expire and no longer login. Timestamp in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub pair_by: Option<String>,
    /// Read only When this DeviceCode was created. Timestamp in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// Read only When this DeviceCode's status was last changed. Timestamp in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub status_changed_at: Option<String>,
    /// Read only When this DeviceCode was paired. Timestamp in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub paired_at: Option<String>,
}
