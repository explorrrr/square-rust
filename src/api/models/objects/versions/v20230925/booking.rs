//! Booking

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::booking_booking_source::BookingBookingSourceV20230925;
use crate::api::models::enums::versions::v20230925::booking_status::BookingStatusV20230925;
use crate::api::models::enums::versions::v20230925::business_appointment_settings_booking_location_type::BusinessAppointmentSettingsBookingLocationTypeV20230925;
use crate::api::models::objects::versions::v20230925::appointment_segment::AppointmentSegmentV20230925;
use crate::api::models::objects::versions::v20230925::booking_creator_details::BookingCreatorDetailsV20230925;

/// Represents a booking as a time-bound service contract for a seller's staff member to provide a specified service at a given location to a requesting customer in one or more appointment segments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingV20230925 {
    /// Read only A unique ID of this object representing a booking.
    /// Max Length 36
    pub id: Option<String>,
    /// The revision number for the booking used for optimistic concurrency.
    pub version: Option<i32>,
    /// Read only The status of the booking, describing where the booking stands with respect to the booking state machine.
    pub status: Option<BookingStatusV20230925>,
    /// Read only The RFC 3339 timestamp specifying the creation time of this booking.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// Read only The RFC 3339 timestamp specifying the most recent update time of this booking.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub updated_at: Option<String>,
    /// The RFC 3339 timestamp specifying the starting time of this booking.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub start_at: Option<String>,
    /// The ID of the Location object representing the location where the booked service is provided. Once set when the booking is created, its value cannot be changed.
    /// Max Length 32
    pub location_id: Option<String>,
    /// The ID of the Customer object representing the customer receiving the booked service.
    /// Max Length 192
    pub customer_id: Option<String>,
    /// The free-text field for the customer to supply notes about the booking. For example, the note can be preferences that cannot be expressed by supported attributes of a relevant CatalogObject instance.
    /// Max Length 4096
    pub customer_note: Option<String>,
    /// The free-text field for the seller to supply notes about the booking. For example, the note can be preferences that cannot be expressed by supported attributes of a specific CatalogObject instance. This field should not be visible to customers.
    /// Max Length 4096
    pub seller_note: Option<String>,
    /// A list of appointment segments for this booking.
    pub appointment_segments: Option<Vec<AppointmentSegmentV20230925>>,
    /// Read only Additional time at the end of a booking. Applications should not make this field visible to customers of a seller.
    pub transition_time_minutes: Option<i32>,
    /// Read only Whether the booking is of a full business day.
    pub all_day: Option<bool>,
    /// The type of location where the booking is held. Access to this field requires seller-level permissions.
    pub location_type: Option<BusinessAppointmentSettingsBookingLocationTypeV20230925>,
    /// Read only Information about the booking creator.
    pub creator_details: Option<BookingCreatorDetailsV20230925>,
    /// Read only The source of the booking. Access to this field requires seller-level permissions.
    pub source: Option<BookingBookingSourceV20230925>,
}
