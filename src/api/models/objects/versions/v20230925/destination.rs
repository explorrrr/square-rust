//! Destination

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::destination_type::DestinationTypeV20230925;

/// Information about the application used to generate a code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationV20230925 {
    /// Type of the destination such as a bank account or debit card.
    pub r#type: Option<DestinationTypeV20230925>,
    /// Square issued unique ID for the `BankAccount` type destination.
    pub id: Option<String>,
}
