//! DataCollectionOptionsInputType enum

use serde::{Deserialize, Serialize};

/// Describes the input type of the data.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataCollectionOptionsInputType {
    /// This value is used to represent an input text that contains a email validation on the client.
    Email,
    /// This value is used to represent an input text that contains a phone number validation on the client.
    PhoneNumber,
}
