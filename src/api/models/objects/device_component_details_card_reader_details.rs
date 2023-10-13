//! DeviceComponentDetailsCardReaderDetails

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceComponentDetailsCardReaderDetails {
    /// The version of the card reader.
    pub version: Option<String>,
}
