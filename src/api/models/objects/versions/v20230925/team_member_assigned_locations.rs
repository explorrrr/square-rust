//! TeamMemberAssignedLocations

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::team_member_assigned_locations_assignment_type::TeamMemberAssignedLocationsAssignmentTypeV20230925;

/// An object that represents a team member's assignment to locations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMemberAssignedLocationsV20230925 {
    /// The current assignment type of the team member.
    pub assignment_type: Option<TeamMemberAssignedLocationsAssignmentTypeV20230925>,
    /// The explicit locations that the team member is assigned to.
    pub location_ids: Option<Vec<String>>,
}
