//! SearchTeamMembersQuery

use serde::{Deserialize, Serialize};

/// Represents the parameters in a search for TeamMember objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTeamMembersQuery {
    /// The options to filter by.
    pub filter: Option<SearchTeamMembersFilter>,
}
