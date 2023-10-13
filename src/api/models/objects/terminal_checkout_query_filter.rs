//! TerminalCheckoutQueryFilter

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalCheckoutQueryFilter {
    /// The TerminalCheckout objects associated with a specific device. If no device is specified, then all TerminalCheckout objects for the merchant are displayed.
    pub device_id: Option<String>,
    /// The time range for the beginning of the reporting period, which is inclusive. Default value: The current time minus one day. Note that TerminalCheckouts are available for 30 days after creation.
    pub created_at: Option<TimeRange>,
    /// Filtered results with the desired status of the TerminalCheckout. Options: PENDING, IN_PROGRESS, CANCEL_REQUESTED, CANCELED, COMPLETED
    pub status: Option<String>,
}
