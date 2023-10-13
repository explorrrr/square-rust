//! DeviceComponentDetailsCardReaderDetails

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceComponentDetailsCardReaderDetailsV20230925 {
    /// The version of the card reader.
    pub version: Option<String>,
}
