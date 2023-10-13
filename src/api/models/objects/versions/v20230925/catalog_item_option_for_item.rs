//! CatalogItemOptionForItem

use serde::{Deserialize, Serialize};

/// An option that can be assigned to an item.
///
/// For example, a t-shirt item may offer a color option or a size option.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogItemOptionForItemV20230925 {
    /// The unique id of the item option, used to form the dimensions of the item option matrix in a specified order.
    pub item_option_id: Option<String>,
}
