//! TenderCardDetails

use serde::{Deserialize, Serialize};

/// Represents additional details of a tender with type CARD or SQUARE_GIFT_CARD
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenderCardDetails {
    /// The credit card payment's current state (such as AUTHORIZED or CAPTURED). See [TenderCardDetailsStatus](https://developer.squareup.com/reference/square/objects/TenderCardDetailsStatus) for possible values.
    pub status: Option<TenderCardDetailsStatus>,
    /// The credit card's non-confidential details.
    pub card: Option<Card>,
    /// The method used to enter the card's details for the transaction.
    pub entry_method: Option<TenderCardDetailsEntryMethod>,
}
