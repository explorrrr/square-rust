//! DestinationDetailsCardRefundDetails

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationDetailsCardRefundDetails {
    /// The card's non-confidential details.
    pub card: Option<Card>,
    /// The method used to enter the card's details for the refund. The method can be KEYED, SWIPED, EMV, ON_FILE, or CONTACTLESS.
    /// Max Length: 50
    pub entry_method: Option<String>,
}
