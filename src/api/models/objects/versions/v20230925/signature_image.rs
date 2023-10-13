//! SignatureImage

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureImageV20230925 {
    /// The mime/type of the image data. Use image/png;base64 for png.
    pub image_type: Option<String>,
    /// The base64 representation of the image.
    pub data: Option<String>,
}
