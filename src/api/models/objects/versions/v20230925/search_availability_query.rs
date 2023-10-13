//! SearchAvailabilityQuery

use serde::{Deserialize, Serialize};

use super::search_availability_filter::SearchAvailabilityFilterV20230925;

/// The query used to search for buyer-accessible availabilities of bookings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAvailabilityQueryV20230925 {
    /// The query filter to search for buyer-accessible availabilities of existing bookings.
    pub filter: SearchAvailabilityFilterV20230925,
}
