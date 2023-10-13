//! LoyaltyProgram

use serde::{Deserialize, Serialize};

/// Represents a Square loyalty program.
///
/// Loyalty programs define how buyers can earn points and redeem points for rewards. Square sellers can have only one loyalty program, which is created and managed from the Seller Dashboard. For more information, see [Loyalty Program Overview](https://developer.squareup.com/docs/loyalty/overview).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyProgram {
    /// The Square-assigned ID of the loyalty program. Updates to the loyalty program do not modify the identifier.
    ///
    /// Max Length 36
    pub id: Option<String>,
    /// Whether the program is currently active.
    pub status: Option<LoyaltyProgramStatus>,
    /// The list of rewards for buyers, sorted by ascending points.
    pub reward_tiers: Option<Vec<LoyaltyProgramRewardTier>>,
    /// If present, details for how points expire.
    pub expiration_policy: Option<LoyaltyProgramExpirationPolicy>,
    /// A cosmetic name for the “points” currency.
    pub terminology: Option<LoyaltyProgramTerminology>,
    /// The [locations](https://developer.squareup.com/reference/square/objects/Location) at which the program is active.
    pub location_ids: Option<Vec<String>>,
    /// The timestamp when the program was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// - UTC: 2020-01-26T02:25:34Z
    /// - Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// The timestamp when the reward was last updated, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// - UTC: 2020-01-26T02:25:34Z
    /// - Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
    /// Defines how buyers can earn loyalty points from the base loyalty program. To check for associated [loyalty promotions](https://developer.squareup.com/reference/square/objects/LoyaltyPromotion) that enable buyers to earn extra points, call [ListLoyaltyPromotions](https://developer.squareup.com/reference/square/loyalty-api/list-loyalty-promotions).
    pub accrual_rules: Option<Vec<LoyaltyProgramAccrualRule>>,
}
