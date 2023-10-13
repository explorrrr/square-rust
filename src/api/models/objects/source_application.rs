//! SourceApplication

use serde::{Deserialize, Serialize};

/// Represents information about the application used to generate a change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceApplication {
    /// Read only The [product](https://developer.squareup.com/reference/square/objects/Product) type of the application.
    pub product: Option<Product>,
    /// Read only The Square-assigned ID of the application. This field is used only if the [product](https://developer.squareup.com/reference/square/objects/Product) type is EXTERNAL_API.
    pub application_id: Option<String>,
    /// Read only The display name of the application (for example, "Custom Application" or "Square POS 4.74 for Android").
    pub name: Option<String>,
}
