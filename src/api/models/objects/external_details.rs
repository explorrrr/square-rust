//! ExternalDetails

use serde::{Deserialize, Serialize};

use crate::api::models::objects::money::Money;
use crate::api::models::enums::external_details_type::ExternalDetailsType;

/// Stores details about an external payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ExternalDetails {
    /// The type of external payment the seller received.
    pub type_: ExternalDetailsType,
    /// A description of the external payment source. For example, "Food Delivery Service".
    pub source: String,
    /// An ID to associate this payment to its originating source.
    pub id: Option<String>,
    /// The fees paid to the source. The amount_money minus this field is the net amount sellers receive.
    pub source_fee_money: Option<Money>,
}
