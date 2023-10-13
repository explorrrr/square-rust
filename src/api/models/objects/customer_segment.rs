//! CustomerSegment

use serde::{Deserialize, Serialize};

/// Represents a group of customer profiles that match one or more predefined filter criteria.
///
/// Segments (also known as Smart Groups) are defined and created within the Customer Directory in the Square Seller Dashboard or Point of Sale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSegment {
    /// A unique Square-generated ID for the segment.
    ///
    /// Max Length 255
    pub id: Option<String>,
    /// The name of the segment.
    pub name: String,
    /// The timestamp when the segment was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// The timestamp when the segment was last updated, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
}
