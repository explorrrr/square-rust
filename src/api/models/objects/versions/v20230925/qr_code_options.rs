//! QrCodeOptions

use serde::{Deserialize, Serialize};

/// Fields to describe the action that displays QR-Codes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrCodeOptionsV20230925 {
    /// The title text to display in the QR code flow on the Terminal.
    ///
    /// Min Length: 1 Max Length: 250
    pub title: String,
    /// The body text to display in the QR code flow on the Terminal.
    ///
    /// Min Length: 1 Max Length: 10000
    pub body: String,
    /// The text representation of the data to show in the QR code as UTF8-encoded data.
    ///
    /// Min Length: 1 Max Length: 1024
    pub barcode_contents: String,
}
