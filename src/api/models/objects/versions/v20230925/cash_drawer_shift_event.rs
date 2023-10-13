//! CashDrawerShiftEvent

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::cash_drawer_event_type::CashDrawerEventTypeV20230925;
use crate::api::models::objects::versions::v20230925::money::MoneyV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashDrawerShiftEventV20230925 {
    /// The unique ID of the event.
    pub id: Option<String>,
    /// The type of cash drawer shift event.
    pub event_type: Option<CashDrawerEventTypeV20230925>,
    /// The amount of money that was added to or removed from the cash drawer in the event. The amount can be positive (for added money) or zero (for other tender type payments). The addition or removal of money can be determined by by the event type.
    pub event_money: Option<MoneyV20230925>,
    /// Read only The event time in RFC 3339 format.
    pub created_at: Option<String>,
    /// An optional description of the event, entered by the employee that created the event.
    pub description: Option<String>,
    /// Read only The ID of the team member that created the event.
    pub team_member_id: Option<String>,
}
