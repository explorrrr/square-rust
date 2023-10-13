//! DeviceComponentDetailsApplicationDetails

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::application_type::ApplicationTypeV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceComponentDetailsApplicationDetailsV20230925 {
    /// The type of application.
    pub application_type: Option<ApplicationTypeV20230925>,
    /// The version of the application.
    pub version: Option<String>,
    /// The location_id of the session for the application.
    pub session_location: Option<String>,
    /// The id of the device code that was used to log in to the device.
    pub device_code_id: Option<String>,
}
