//! SearchTeamMembersQuery

use serde::{Deserialize, Serialize};

use super::search_team_members_filter::SearchTeamMembersFilterV20230925;

/// Represents the parameters in a search for TeamMember objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchTeamMembersQueryV20230925 {
    /// The options to filter by.
    pub filter: Option<SearchTeamMembersFilterV20230925>,
}
