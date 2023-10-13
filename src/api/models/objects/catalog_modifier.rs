//! CatalogModifier

use serde::{Deserialize, Serialize};

/// A modifier applicable to items at the time of sale.
/// An example of a modifier is a Cheese add-on to a Burger item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogModifier {
    /// The modifier name. This is a searchable attribute for use in applicable query filters, and its value length is of Unicode code points.
    /// Max Length 255
    pub name: Option<String>,
    /// The modifier price.
    pub price_money: Option<Money>,
    /// Determines where this CatalogModifier appears in the CatalogModifierList.
    pub ordinal: Option<i32>,
    /// The ID of the CatalogModifierList associated with this modifier.
    pub modifier_list_id: Option<String>,
    /// Location-specific price overrides.
    pub location_overrides: Option<Vec<ModifierLocationOverrides>>,
    /// The ID of the image associated with this CatalogModifier instance. Currently this image is not displayed by Square, but is free to be displayed in 3rd party applications.
    pub image_id: Option<String>,
}
