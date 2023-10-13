//! TeamMember

use serde::{Deserialize, Serialize};

/// A record representing an individual team member for a business.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    /// The unique ID for the team member.
    pub id: Option<String>,
    /// A second ID used to associate the team member with an entity in another system.
    pub reference_id: Option<String>,
    /// Whether the team member is the owner of the Square account.
    pub is_owner: Option<bool>,
    /// Describes the status of the team member.
    pub status: Option<TeamMemberStatus>,
    /// The given name (that is, the first name) associated with the team member.
    pub given_name: Option<String>,
    /// The family name (that is, the last name) associated with the team member.
    pub family_name: Option<String>,
    /// The email address associated with the team member.
    pub email_address: Option<String>,
    /// The team member's phone number, in E.164 format. For example: +14155552671 - the country code is 1 for US +551155256325 - the country code is 55 for BR
    pub phone_number: Option<String>,
    /// The timestamp, in RFC 3339 format, describing when the team member was created. For example, "2018-10-04T04:00:00-07:00" or "2019-02-05T12:00:00Z".
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// The timestamp, in RFC 3339 format, describing when the team member was last updated. For example, "2018-10-04T04:00:00-07:00" or "2019-02-05T12:00:00Z".
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
    /// Describes the team member's assigned locations.
    pub assigned_locations: Option<TeamMemberAssignedLocations>,
}
