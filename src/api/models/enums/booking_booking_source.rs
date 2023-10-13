//! BookingBookingSource Enum

use serde::{Deserialize, Serialize};

/// Supported sources a booking was created from.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BookingBookingSource {
    /// The booking was created by a seller from a Square Appointments application, such as the Square Appointments Dashboard or a Square Appointments mobile app.
    FirstPartyMerchant,
    /// The booking was created by a buyer from a Square Appointments application, such as Square Online Booking Site.
    FirstPartyBuyer,
    /// The booking was created by a buyer created from a third-party application.
    ThirdPartyBuyer,
    /// The booking was created by a seller or a buyer from the Square Bookings API.
    Api,
}
