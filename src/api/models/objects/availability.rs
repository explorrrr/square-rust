//! Availability

use crate::api::models::objects::appointment_segment::AppointmentSegment;

use serde::{Deserialize, Serialize};

/// Defines an appointment slot that encapsulates the appointment segments, location and starting time available for booking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Availability {
    /// The RFC 3339 timestamp specifying the beginning time of the slot available for booking.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub start_at: Option<String>,
    /// Read only The ID of the location available for booking.
    /// Max Length 32
    pub location_id: Option<String>,
    /// The list of appointment segments available for booking
    pub appointment_segments: Option<Vec<AppointmentSegment>>,
}
