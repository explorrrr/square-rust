//! BusinessAppointmentSettingsBookingLocationType enum

use serde::{Deserialize, Serialize};

/// Supported types of location where service is provided.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessAppointmentSettingsBookingLocationTypeV20230925 {
    /// The service is provided at a seller location.
    BusinessLocation,
    /// The service is provided at a customer location.
    CustomerLocation,
    /// The service is provided over the phone.
    Phone,
}
