//! SearchAvailabilityQuery

use serde::{Deserialize, Serialize};

/// The query used to search for buyer-accessible availabilities of bookings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAvailabilityQuery {
    /// The query filter to search for buyer-accessible availabilities of existing bookings.
    pub filter: SearchAvailabilityFilter,
}
