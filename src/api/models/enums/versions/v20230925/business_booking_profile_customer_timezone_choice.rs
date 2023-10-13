//! BusinessBookingProfileCustomerTimezoneChoice enum

use serde::{Deserialize, Serialize};

/// Choices of customer-facing time zone used for bookings.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessBookingProfileCustomerTimezoneChoiceV20230925 {
    /// Use the time zone of the business location for bookings.
    BusinessLocationTimezone,
    /// Use the customer-chosen time zone for bookings.
    CustomerChoice,
}
