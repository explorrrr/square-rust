//! EcomVisibility Enum

use serde::{Deserialize, Serialize};

/// EcomVisibility enum.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EcomVisibilityV20230925 {
    /// Item is not synced with Ecom (Weebly). This is the default state
    Unindexed,
    /// Item is synced but is unavailable within Ecom (Weebly) and Online Checkout
    Unavailable,
    /// Option for seller to choose manually created Quick Amounts.
    Hidden,
    /// Item is synced but available within Ecom (Weebly) and Online Checkout but is hidden from Ecom Store.
    Visible,
}
