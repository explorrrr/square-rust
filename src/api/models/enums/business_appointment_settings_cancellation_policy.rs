//! BusinessAppointmentSettingsCancellationPolicy enum

use serde::{Deserialize, Serialize};

/// The category of the seller’s cancellation policy.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessAppointmentSettingsCancellationPolicy {
    /// Cancellations are treated as no shows and may incur a fee as specified by cancellation_fee_money.
    CancellationTreatedAsNoShow,
    /// Cancellations follow the seller-specified policy that is described in free-form text and not enforced automatically by Square.
    CustomPolicy,
}
