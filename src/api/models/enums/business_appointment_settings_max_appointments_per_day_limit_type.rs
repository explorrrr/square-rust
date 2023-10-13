//! BusinessAppointmentSettingsMaxAppointmentsPerDayLimitType enum

use serde::{Deserialize, Serialize};

/// Types of daily appointment limits.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessAppointmentSettingsMaxAppointmentsPerDayLimitType {
    /// The maximum number of daily appointments is set on a per team member basis.
    PerTeamMember,
    /// The maximum number of daily appointments is set on a per location basis.
    PerLocation,
}
