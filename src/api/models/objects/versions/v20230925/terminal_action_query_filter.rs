//! TerminalActionQueryFilter

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::terminal_action_action_type::TerminalActionActionTypeV20230925;

use super::time_range::TimeRangeV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalActionQueryFilterV20230925 {
    /// The device ID for the terminal.
    pub device_id: Option<String>,
    /// Time range for the beginning of the reporting period. Inclusive. Default value: The current time minus one day. Note that TerminalActions are available for 30 days after creation.
    pub created_at: Option<TimeRangeV20230925>,
    /// Filter results with the desired status of the TerminalAction Options: PENDING, IN_PROGRESS, CANCEL_REQUESTED, CANCELED, COMPLETED
    pub status: Option<String>,
    /// Filter results with the requested ActionType.
    pub r#type: Option<TerminalActionActionTypeV20230925>,
}
