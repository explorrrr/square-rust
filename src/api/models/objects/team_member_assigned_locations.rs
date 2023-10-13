//! TeamMemberAssignedLocations

use serde::{Deserialize, Serialize};

/// An object that represents a team member's assignment to locations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMemberAssignedLocations {
    /// The current assignment type of the team member.
    pub assignment_type: Option<TeamMemberAssignedLocationsAssignmentType>,
    /// The explicit locations that the team member is assigned to.
    pub location_ids: Option<Vec<String>>,
}
