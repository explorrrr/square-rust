//! DisputeState Enum

use serde::{Deserialize, Serialize};

/// The list of possible dispute states.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DisputeStateV20230925 {
    /// The initial state of an inquiry with evidence required
    InquiryEvidenceRequired,
    /// Inquiry evidence has been submitted and the bank is processing the inquiry
    InquiryProcessing,
    /// The inquiry is complete
    InquiryClosed,
    /// The initial state of a dispute with evidence required
    EvidenceRequired,
    /// Dispute evidence has been submitted and the bank is processing the dispute
    Processing,
    /// The bank has completed processing the dispute and the seller has won
    Won,
    /// The bank has completed processing the dispute and the seller has lost
    Lost,
    /// The seller has accepted the dispute
    Accepted,
}
