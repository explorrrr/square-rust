//! DestinationDetailsCardRefundDetails

use serde::{Deserialize, Serialize};

use super::card::CardV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationDetailsCardRefundDetailsV20230925 {
    /// The card's non-confidential details.
    pub card: Option<CardV20230925>,
    /// The method used to enter the card's details for the refund. The method can be KEYED, SWIPED, EMV, ON_FILE, or CONTACTLESS.
    /// Max Length: 50
    pub entry_method: Option<String>,
}
