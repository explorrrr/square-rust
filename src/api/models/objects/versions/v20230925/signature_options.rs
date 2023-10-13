//! SignatureOptions

use serde::{Deserialize, Serialize};

use super::signature_image::SignatureImageV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureOptionsV20230925 {
    /// The title text to display in the signature capture flow on the Terminal.
    ///
    /// Min Length 1 Max Length 250
    pub title: String,
    /// The body text to display in the signature capture flow on the Terminal.
    ///
    /// Min Length 1 Max Length 10000
    pub body: String,
    /// The signature image data that can be signed by a buyer on a Square Terminal.
    pub signature: Option<Vec<SignatureImageV20230925>>,
}
