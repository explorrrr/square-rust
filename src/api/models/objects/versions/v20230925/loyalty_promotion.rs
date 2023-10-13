//! LoyaltyPromotion

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::loyalty_promotion_status::LoyaltyPromotionStatusV20230925;

use super::{loyalty_promotion_incentive::LoyaltyPromotionIncentiveV20230925, loyalty_promotion_available_time_data::LoyaltyPromotionAvailableTimeDataV20230925, loyalty_promotion_trigger_limit::LoyaltyPromotionTriggerLimitV20230925, money::MoneyV20230925};

/// Represents a promotion for a [loyalty program](https://developer.squareup.com/reference/square/objects/LoyaltyProgram).
///
/// Loyalty promotions enable buyers to earn extra points on top of those earned from the base program.
///
/// A loyalty program can have a maximum of 10 loyalty promotions with an `ACTIVE` or `SCHEDULED` status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyPromotionV20230925 {
    /// The Square-assigned ID of the promotion.
    ///
    /// Min Length: 1 Max Length: 255
    pub id: Option<String>,
    /// The name of the promotion.
    ///
    /// Min Length: 1 Max Length: 50
    pub name: String,
    /// The points incentive for the promotion. This field defines whether promotion points are earned by multiplying base program points or by adding a specified number of points.
    pub incentive: LoyaltyPromotionIncentiveV20230925,
    /// The scheduling information that defines when purchases can qualify to earn points from an `ACTIVE` promotion.
    pub available_time: LoyaltyPromotionAvailableTimeDataV20230925,
    /// The number of times a buyer can earn promotion points during a specified interval. If not specified, buyers can trigger the promotion an unlimited number of times.
    pub trigger_limit: Option<LoyaltyPromotionTriggerLimitV20230925>,
    /// The current status of the promotion.
    pub status: LoyaltyPromotionStatusV20230925,
    /// The timestamp of when the promotion was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// The timestamp of when the promotion was canceled, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub canceled_at: Option<String>,
    /// The timestamp when the promotion was last updated, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34
    pub updated_at: Option<String>,
    /// The ID of the [loyalty program](https://developer.squareup.com/reference/square/objects/LoyaltyProgram) associated with the promotion.
    pub loyalty_program_id: Option<String>,
    /// The minimum purchase amount required to earn promotion points. If specified, this amount is positive.
    pub minimum_spend_amount_money: Option<MoneyV20230925>,
    /// The IDs of any qualifying ITEM_VARIATION [catalog objects](https://developer.squareup.com/reference/square/objects/CatalogObject). If specified, the purchase must include at least one of these items to qualify for the promotion.
    ///
    /// This option is valid only if the base loyalty program uses a `VISIT` or `SPEND` accrual rule. With `SPEND` accrual rules, make sure that qualifying promotional items are not excluded.
    ///
    /// You can specify `qualifying_item_variation_ids` or `qualifying_category_ids` for a given promotion, but not both.
    pub qualifying_item_variation_ids: Option<Vec<String>>,
    /// The IDs of any qualifying CATEGORY [catalog objects](https://developer.squareup.com/reference/square/objects/CatalogObject). If specified, the purchase must include at least one item from one of these categories to qualify for the promotion.
    ///
    /// This option is valid only if the base loyalty program uses a `VISIT` or `SPEND` accrual rule. With `SPEND` accrual rules, make sure that qualifying promotional items are not excluded.
    ///
    /// You can specify `qualifying_category_ids` or `qualifying_item_variation_ids` for a promotion, but not both.
    pub qualifying_category_ids: Option<Vec<String>>,
}
