//! BusinessAppointmentSettings

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::business_appointment_settings_alignment_time::BusinessAppointmentSettingsAlignmentTimeV20230925;
use crate::api::models::enums::versions::v20230925::business_appointment_settings_booking_location_type::BusinessAppointmentSettingsBookingLocationTypeV20230925;
use crate::api::models::enums::versions::v20230925::business_appointment_settings_cancellation_policy::BusinessAppointmentSettingsCancellationPolicyV20230925;
use crate::api::models::enums::versions::v20230925::business_appointment_settings_max_appointments_per_day_limit_type::BusinessAppointmentSettingsMaxAppointmentsPerDayLimitTypeV20230925;
use crate::api::models::objects::versions::v20230925::money::MoneyV20230925;

/// The service appointment settings, including where and how the service is provided.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessAppointmentSettingsV20230925 {
    /// Types of the location allowed for bookings.
    pub location_types: Option<Vec<BusinessAppointmentSettingsBookingLocationTypeV20230925>>,
    /// The time unit of the service duration for bookings.
    pub alignment_time: Option<BusinessAppointmentSettingsAlignmentTimeV20230925>,
    /// The minimum lead time in seconds before a service can be booked. A booking must be created at least this amount of time before its starting time.
    pub min_booking_lead_time_seconds: Option<i32>,
    /// The maximum lead time in seconds before a service can be booked. A booking must be created at most this amount of time before its starting time.
    pub max_booking_lead_time_seconds: Option<i32>,
    /// Indicates whether a customer can choose from all available time slots and have a staff member assigned automatically (true) or not (false).
    pub any_team_member_booking_enabled: Option<bool>,
    /// Indicates whether a customer can book multiple services in a single online booking.
    pub multiple_service_booking_enabled: Option<bool>,
    /// Indicates whether the daily appointment limit applies to team members or to business locations.
    pub max_appointments_per_day_limit_type: Option<BusinessAppointmentSettingsMaxAppointmentsPerDayLimitTypeV20230925>,
    /// The maximum number of daily appointments per team member or per location.
    pub max_appointments_per_day_limit: Option<i32>,
    /// The cut-off time in seconds for allowing clients to cancel or reschedule an appointment.
    pub cancellation_window_seconds: Option<i32>,
    /// The flat-fee amount charged for a no-show booking.
    pub cancellation_fee_money: Option<MoneyV20230925>,
    /// The cancellation policy adopted by the seller.
    pub cancellation_policy: Option<BusinessAppointmentSettingsCancellationPolicyV20230925>,
    /// The free-form text of the seller's cancellation policy.
    /// Max Length 65536
    pub cancellation_policy_text: Option<String>,
    /// Indicates whether customers has an assigned staff member (true) or can select s staff member of their choice (false).
    pub skip_booking_flow_staff_selection: Option<bool>,
}
