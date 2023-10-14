//! ExternalDetails

use serde::{Deserialize, Serialize};

use super::money::MoneyV20230925;
use crate::api::models::enums::versions::v20230925::external_details_type::ExternalDetailsTypeV20230925;

/// Stores details about an external payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalDetailsV20230925 {
    /// The type of external payment the seller received.
    pub r#type: ExternalDetailsTypeV20230925,
    /// A description of the external payment source. For example, "Food Delivery Service".
    pub source: String,
    /// An ID to associate this payment to its originating source.
    pub id: Option<String>,
    /// The fees paid to the source. The amount_money minus this field is the net amount sellers receive.
    pub source_fee_money: Option<MoneyV20230925>,
}
