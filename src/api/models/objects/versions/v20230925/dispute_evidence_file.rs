//! DisputeEvidenceFile

use serde::{Deserialize, Serialize};

/// A file to be uploaded as dispute evidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisputeEvidenceFileV20230925 {
    /// The file name including the file extension. For example: "receipt.tiff".
    ///
    /// Min Length 1 Max Length 40
    pub filename: Option<String>,
    /// Dispute evidence files must be application/pdf, image/heic, image/heif, image/jpeg, image/png, or image/tiff formats.
    ///
    /// Min Length 1 Max Length 40
    pub filetype: Option<String>,
}
