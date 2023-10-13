//! CatalogQuickAmount

use serde::{Deserialize, Serialize};

use crate::api::models::objects::versions::v20230925::money::MoneyV20230925;

/// Represents a Quick Amount in the Catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogQuickAmountV20230925 {
    /// Represents the type of the Quick Amount.
    pub r#type: String,
    /// Represents the actual amount of the Quick Amount with Money type.
    pub amount: MoneyV20230925,
    /// Describes the ranking of the Quick Amount provided by machine learning model, in the range [0, 100]. MANUAL type amount will always have score = 100.
    pub score: Option<i64>,
    /// The order in which this Quick Amount should be displayed.
    pub ordinal: Option<i64>,
}
