//! BusinessAppointmentSettingsAlignmentTime enum

use serde::{Deserialize, Serialize};

/// Defines the alignment time units of a service duration for bookings.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessAppointmentSettingsAlignmentTimeV20230925 {
    /// The service duration unit is one visit of a fixed time interval specified by the seller.
    ServiceDuration,
    /// The service duration unit is a 15-minute interval. Bookings can be scheduled every quarter hour.
    QuarterHourly,
    /// The service duration unit is a 30-minute interval. Bookings can be scheduled every half hour.
    HalfHourly,
    /// The service duration unit is a 60-minute interval. Bookings can be scheduled every hour.
    Hourly,
}
