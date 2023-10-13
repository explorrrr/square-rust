//! DestinationDetails

use serde::{Deserialize, Serialize};

/// Details about a refund's destination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationDetails {
    /// Details about a card refund. Only populated if the destination_type is CARD.
    pub card_details: Option<DestinationDetailsCardRefundDetails>,
}
