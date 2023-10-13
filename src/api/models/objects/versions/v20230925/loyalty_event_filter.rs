//! LoyaltyEventFilter

use serde::{Deserialize, Serialize};

use super::{loyalty_envent_loyalty_account_filter::LoyaltyEventLoyaltyAccountFilterV20230925, loyalty_event_type_filter::LoyaltyEventTypeFilterV20230925, loyalty_event_location_filter::LoyaltyEventLocationFilterV20230925, loyalty_event_order_filter::LoyaltyEventOrderFilterV20230925, loyalty_event_date_time_filter::LoyaltyEventDateTimeFilterV20230925};

/// The filtering criteria.
///
/// If the request specifies multiple filters, the endpoint uses a logical AND to evaluate them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventFilterV20230925 {
    /// Filter events by loyalty account.
    pub loyalty_account_filter: Option<LoyaltyEventLoyaltyAccountFilterV20230925>,
    /// Filter events by event type.
    pub type_filter: Option<LoyaltyEventTypeFilterV20230925>,
    /// Filter events by date time range. For each range, the start time is inclusive and the end time is exclusive.
    pub date_time_filter: Option<LoyaltyEventDateTimeFilterV20230925>,
    /// Filter events by location.
    pub location_filter: Option<LoyaltyEventLocationFilterV20230925>,
    /// Filter events by the order associated with the event.
    pub order_filter: Option<LoyaltyEventOrderFilterV20230925>,
}
