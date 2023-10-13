//! SearchVendorsRequestSortField Enum

use serde::{Deserialize, Serialize};

/// The field to sort the returned [`Vendor`](https://developer.squareup.com/reference/square/objects/Vendor) objects by.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SearchVendorsRequestSortField {
    /// To sort the result by the name of the [`Vendor`](https://developer.squareup.com/reference/square/objects/Vendor) objects.
    Name,
    /// To sort the result by the creation time of the [`Vendor`](https://developer.squareup.com/reference/square/objects/Vendor) objects.
    CreatedAt,
}
