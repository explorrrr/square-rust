//! CatalogItemModifierListInfo

use serde::{Deserialize, Serialize};

use crate::api::models::objects::versions::v20230925::catalog_modifier_override::CatalogModifierOverrideV20230925;

/// Options to control the properties of a CatalogModifierList applied to a CatalogItem instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogItemModifierListInfoV20230925 {
    /// The ID of the CatalogModifierList controlled by this CatalogModifierListInfo.
    /// Min Length 1
    pub modifier_list_id: String,
    /// A set of CatalogModifierOverride objects that override whether a given CatalogModifier is enabled by default.
    pub modifier_overrides: Option<Vec<CatalogModifierOverrideV20230925>>,
    /// If 0 or larger, the smallest number of CatalogModifiers that must be selected from this CatalogModifierList.
    pub min_selected_modifiers: Option<i32>,
    /// If 0 or larger, the largest number of CatalogModifiers that can be selected from this CatalogModifierList.
    pub max_selected_modifiers: Option<i32>,
    /// If true, enable this CatalogModifierList. The default value is true.
    pub enabled: Option<bool>,
}
