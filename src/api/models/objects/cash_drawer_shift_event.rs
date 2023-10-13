//! CashDrawerShiftEvent

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashDrawerShiftEvent {
    /// The unique ID of the event.
    pub id: Option<String>,
    /// The type of cash drawer shift event.
    pub event_type: Option<CashDrawerEventType>,
    /// The amount of money that was added to or removed from the cash drawer in the event. The amount can be positive (for added money) or zero (for other tender type payments). The addition or removal of money can be determined by by the event type.
    pub event_money: Option<Money>,
    /// Read only The event time in RFC 3339 format.
    pub created_at: Option<String>,
    /// An optional description of the event, entered by the employee that created the event.
    pub description: Option<String>,
    /// Read only The ID of the team member that created the event.
    pub team_member_id: Option<String>,
}
