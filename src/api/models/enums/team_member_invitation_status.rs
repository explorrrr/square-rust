//! TeamMemberInvitationStatus Enum

use serde::{Deserialize, Serialize};

/// Enumerates the possible invitation statuses the team member can have within a business.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TeamMemberInvitationStatus {
    /// The team member has not received an invitation.
    Uninvited,
    /// The team member has received an invitation, but had not accepted it.
    Pending,
    /// The team member has both received and accepted an invitation.
    Accepted,
}
