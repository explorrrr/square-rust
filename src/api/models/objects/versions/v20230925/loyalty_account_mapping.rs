//! LoyaltyAccountMapping

use serde::{Deserialize, Serialize};

/// Represents the mapping that associates a loyalty account with a buyer.
///
/// Currently, a loyalty account can only be mapped to a buyer by phone number. For more information, see [Loyalty Overview](https://developer.squareup.com/docs/loyalty/overview).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyAccountMappingV20230925 {
    /// Read only The Square-assigned ID of the mapping.
    ///
    /// Max Length 36
    pub id: Option<String>,
    /// Read only The timestamp when the mapping was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// The phone number of the buyer, in E.164 format. For example, "+14155551111".
    pub phone_number: Option<String>,
}
