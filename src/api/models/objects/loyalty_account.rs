//! LoyaltyAccount

use serde::{Deserialize, Serialize};

/// Describes a loyalty account in a [loyalty program](https://developer.squareup.com/reference/square/objects/LoyaltyProgram).
///
/// For more information, see [Create and Retrieve Loyalty Accounts](https://developer.squareup.com/docs/loyalty-api/loyalty-accounts).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyAccount {
    /// Read only The Square-assigned ID of the loyalty account.
    ///
    /// Max Length 36
    pub id: Option<String>,
    /// Read only The Square-assigned ID of the [loyalty program](https://developer.squareup.com/reference/square/objects/LoyaltyProgram ) to which the account belongs.
    ///
    /// Min Length 1 Max Length 36
    pub program_id: String,
    /// Read only The available point balance in the loyalty account. If points are scheduled to expire, they are listed in the `expiring_point_deadlines` field.
    ///
    /// Your application should be able to handle loyalty accounts that have a negative point balance (balance is less than 0). This might occur if a seller makes a manual adjustment or as a result of a refund or exchange.
    pub balance: Option<i32>,
    /// Read only The total points accrued during the lifetime of the account.
    pub lifetime_points: Option<i32>,
    /// The Square-assigned ID of the [customer](https://developer.squareup.com/reference/square/objects/Customer) that is associated with the account.
    pub customer_id: Option<String>,
    /// The timestamp when the buyer joined the loyalty program, in RFC 3339 format. This field is used to display the `Enrolled On` or `Member Since` date in first-party Square products.
    ///
    /// If this field is not set in a `CreateLoyaltyAccount` request, Square populates it after the buyer's first action on their account (when `AccumulateLoyaltyPoints` or `CreateLoyaltyReward` is called). In first-party flows, Square populates the field when the buyer agrees to the terms of service in Square Point of Sale.
    ///
    /// This field is typically specified in a `CreateLoyaltyAccount` request when creating a loyalty account for a buyer who already interacted with their account. For example, you would set this field when migrating accounts from an external system. The timestamp in the request can represent a current or previous date and time, but it cannot be set for the future.
    ///
    /// Examples for January 25th, 2020 6:25:34
    ///
    /// Pacific Standard Time: 2020-01-25T18:25:34-08:00
    pub enrolled_at: Option<String>,
    /// Read only The timestamp when the loyalty account was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// Read only The timestamp when the loyalty account was last updated, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
    /// The mapping that associates the loyalty account with a buyer. Currently, a loyalty account can only be mapped to a buyer by phone number.
    ///
    /// To create a loyalty account, you must specify the `mapping` field, with the buyer's phone number in the `phone_number` field.
    pub mapping: Option<LoyaltyAccountMapping>,
    /// The schedule for when points expire in the loyalty account balance. This field is present only if the account has points that are scheduled to expire.
    ///
    /// The total number of points in this field equals the number of points in the `balance` field.
    pub expiring_point_deadlines: Option<Vec<LoyaltyAccountExpiringPointDeadline>>,
}
