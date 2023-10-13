//! LoyaltyEventFilter

use serde::{Deserialize, Serialize};

/// The filtering criteria.
///
/// If the request specifies multiple filters, the endpoint uses a logical AND to evaluate them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyEventFilter {
    /// Filter events by loyalty account.
    pub loyalty_account_filter: Option<LoyaltyEventLoyaltyAccountFilter>,
    /// Filter events by event type.
    pub type_filter: Option<LoyaltyEventTypeFilter>,
    /// Filter events by date time range. For each range, the start time is inclusive and the end time is exclusive.
    pub date_time_filter: Option<LoyaltyEventDateTimeFilter>,
    /// Filter events by location.
    pub location_filter: Option<LoyaltyEventLocationFilter>,
    /// Filter events by the order associated with the event.
    pub order_filter: Option<LoyaltyEventOrderFilter>,
}
