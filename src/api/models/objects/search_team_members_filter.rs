//! SearchTeamMembersFilter

use serde::{Deserialize, Serialize};

/// Represents a filter used in a search for TeamMember objects.
///
/// AND logic is applied between the individual fields, and OR logic is applied within list-based fields. For example, setting this filter value:
///
/// ```no_run
/// filter = (locations_ids = ["A", "B"], status = ACTIVE)
/// ```
/// returns only active team members assigned to either location "A" or "B".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTeamMembersFilter {
    /// When present, filters by team members assigned to the specified locations. When empty, includes team members assigned to any location.
    pub location_ids: Option<Vec<String>>,
    /// When present, filters by team members who match the given status. When empty, includes team members of all statuses.
    pub status: Option<TeamMemberStatus>,
    /// When present and set to true, returns the team member who is the owner of the Square account.
    pub is_owner: Option<bool>,
}
