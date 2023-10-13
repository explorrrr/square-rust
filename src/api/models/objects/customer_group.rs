//! CustomerGroup

use serde::{Deserialize, Serialize};

/// Represents a group of customer profiles.
///
/// Customer groups can be created, be modified, and have their membership defined using the Customers API or within the Customer Directory in the Square Seller Dashboard or Point of Sale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerGroup {
    /// A unique Square-generated ID for the customer group.
    ///
    /// Max Length 255
    pub id: Option<String>,
    /// The name of the customer group.
    pub name: String,
    /// The timestamp when the customer group was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// The timestamp when the customer group was last updated, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
}
