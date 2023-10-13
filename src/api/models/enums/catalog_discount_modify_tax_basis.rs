//! CatalogDiscountModifyTaxBasis Enum

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogDiscountModifyTaxBasis {
    /// Application of the discount will modify the tax basis.
    ModifyTaxBasis,
    /// Application of the discount will not modify the tax basis.
    DoNotModifyTaxBasis,
}
