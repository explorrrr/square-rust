//! TenderCardDetails

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::{tender_card_details_status::TenderCardDetailsStatusV20230925, tender_card_details_entry_method::TenderCardDetailsEntryMethodV20230925};

use super::card::CardV20230925;

/// Represents additional details of a tender with type CARD or SQUARE_GIFT_CARD
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenderCardDetailsV20230925 {
    /// The credit card payment's current state (such as AUTHORIZED or CAPTURED). See [TenderCardDetailsStatus](https://developer.squareup.com/reference/square/objects/TenderCardDetailsStatus) for possible values.
    pub status: Option<TenderCardDetailsStatusV20230925>,
    /// The credit card's non-confidential details.
    pub card: Option<CardV20230925>,
    /// The method used to enter the card's details for the transaction.
    pub entry_method: Option<TenderCardDetailsEntryMethodV20230925>,
}
