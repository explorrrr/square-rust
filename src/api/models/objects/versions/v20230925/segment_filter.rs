//! SegmentFilter

use serde::{Deserialize, Serialize};

use super::filter_value::FilterValueV20230925;

/// A query filter to search for buyer-accessible appointment segments by.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentFilterV20230925 {
    /// The ID of the [CatalogItemVariation](https://developer.squareup.com/reference/square/objects/CatalogItemVariation) object representing the service booked in this segment.
    /// Min Length 1
    /// Max Length 36
    pub service_variation_id: String,
    /// A query filter to search for buyer-accessible appointment segments with service-providing team members matching the specified list of team member IDs. Supported query expressions are
    /// - ANY: return the appointment segments with team members whose IDs match any member in this list.
    /// - NONE: return the appointment segments with team members whose IDs are not in this list.
    /// - ALL: not supported.
    /// When no expression is specified, any service-providing team member is eligible to fulfill the Booking.
    pub team_member_id_filter: Option<FilterValueV20230925>,
}
