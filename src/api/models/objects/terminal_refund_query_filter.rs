//! TerminalRefundQueryFilter

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalRefundQueryFilter {
    /// The device ID of the device associated with the TerminalRefund.
    pub device_id: Option<String>,
    /// The timestamp for the beginning of the reporting period, in RFC 3339 format. Inclusive. Default value: The current time minus one day. Note that TerminalRefunds are available for 30 days after creation.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<TimeRange>,
    /// Filtered results with the desired status of the TerminalRefund. Options: PENDING, IN_PROGRESS, CANCEL_REQUESTED, CANCELED, or COMPLETED.
    pub status: Option<String>,
}
