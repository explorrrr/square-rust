//! CatalogCategory

use serde::{Deserialize, Serialize};

/// A category to which a CatalogItem instance belongs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogCategory {
    /// The category name. This is a searchable attribute for use in applicable query filters, and its value length is of Unicode code points.
    /// Max Length 255
    pub name: Option<String>,
    /// The IDs of images associated with this CatalogCategory instance. Currently these images are not displayed by Square, but are free to be displayed in 3rd party applications.
    pub image_ids: Option<Vec<String>>,
}
