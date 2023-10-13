//! LoyaltyProgramRewardTier

use serde::{Deserialize, Serialize};

/// Represents a reward tier in a loyalty program.
///
/// A reward tier defines how buyers can redeem points for a reward, such as the number of points required and the value and scope of the discount. A loyalty program can offer multiple reward tiers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyProgramRewardTier {
    /// The Square-assigned ID of the reward tier.
    ///
    /// Max Length 36
    pub id: Option<String>,
    /// The points exchanged for the reward tier.
    ///
    /// Min 1
    pub points: i32,
    /// The name of the reward tier.
    pub name: Option<String>,
    /// The timestamp when the reward tier was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// A reference to the specific version of a PRICING_RULE catalog object that contains information about the reward tier discount.
    ///
    /// Use object_id and catalog_version with the RetrieveCatalogObject endpoint to get discount details. Make sure to set include_related_objects to true in the request to retrieve all catalog objects that define the discount. For more information, see Getting discount details for a reward tier.
    pub pricing_rule_reference: CatalogObjectReference,
}
