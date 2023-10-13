//! DeviceComponentDetailsApplicationDetails

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceComponentDetailsApplicationDetails {
    /// The type of application.
    pub application_type: Option<ApplicationType>,
    /// The version of the application.
    pub version: Option<String>,
    /// The location_id of the session for the application.
    pub session_location: Option<String>,
    /// The id of the device code that was used to log in to the device.
    pub device_code_id: Option<String>,
}
