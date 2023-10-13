//! DestinationDetails

use serde::{Deserialize, Serialize};

use super::destination_details_card_refund_details::DestinationDetailsCardRefundDetailsV20230925;

/// Details about a refund's destination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationDetailsV20230925 {
    /// Details about a card refund. Only populated if the destination_type is CARD.
    pub card_details: Option<DestinationDetailsCardRefundDetailsV20230925>,
}
