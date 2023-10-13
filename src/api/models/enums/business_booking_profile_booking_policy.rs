//! BusinessBookingProfileBookingPolicy enum

use serde::{Deserialize, Serialize};

/// Policies for accepting bookings.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessBookingProfileBookingPolicy {
    /// The seller accepts all booking requests automatically.
    AcceptAll,
    /// The seller must accept requests to complete bookings.
    RequiresAcceptance,
}
