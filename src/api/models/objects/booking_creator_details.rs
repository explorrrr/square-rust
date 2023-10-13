//! BookingCreatorDetails

use serde::{Deserialize, Serialize};
use crate::api::models::enums::booking_creator_details_creator_type::BookingCreatorDetailsCreatorType;

/// Information about a booking creator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingCreatorDetails {
    /// Read only The seller-accessible type of the creator of the booking.
    pub creator_type: Option<BookingCreatorDetailsCreatorType>,
    /// Read only The ID of the team member who created the booking, when the booking creator is of the TEAM_MEMBER type. Access to this field requires seller-level permissions.
    /// Max Length 32
    pub team_member_id: Option<String>,
    /// Read only The ID of the customer who created the booking, when the booking creator is of the CUSTOMER type. Access to this field requires seller-level permissions.
    /// Max Length 192
    pub customer_id: Option<String>,
}
