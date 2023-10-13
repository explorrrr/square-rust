//! SubscriptionCadence Enum

use serde::{Deserialize, Serialize};

/// Determines the billing cadence of a [Subscription](https://developer.squareup.com/reference/square/objects/Subscription)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionCadenceV20230925 {
    /// Once per day
    Daily,
    /// Once per week
    Weekly,
    /// Every two weeks
    EveryTwoWeeks,
    /// Once every 30 days
    ThirtyDays,
    /// Once every 60 days
    SixtyDays,
    /// Once every 90 days
    NinetyDays,
    /// Once per month
    Monthly,
    /// Once every two months
    EveryTwoMonths,
    /// Once every three months
    Quarterly,
    /// Once every four months
    EveryFourMonths,
    /// Once every six months
    EverySixMonths,
    /// Once per year
    Annual,
    /// Once every two years
    EveryTwoYears,
}
