//! MerchantStatus Enum

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MerchantStatusV20230925 {
    /// A fully operational merchant account. The merchant can interact with Square products and APIs.
    Active,
    /// A functionally limited merchant account. The merchant can only have limited interaction via Square APIs. The merchant cannot log in or access the seller dashboard.
    Inactive,
}
