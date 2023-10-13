//! Destination

use serde::{Deserialize, Serialize};

/// Information about the application used to generate a code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Destination {
    /// Type of the destination such as a bank account or debit card.
    pub r#type: Option<DestinationType>,
    /// Square issued unique ID for the `BankAccount` type destination.
    pub id: Option<String>,
}
