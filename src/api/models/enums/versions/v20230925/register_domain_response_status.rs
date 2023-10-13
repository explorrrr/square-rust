//! RegisterDomainResponseStatus Enum

use serde::{Deserialize, Serialize};

/// The status of the domain registration.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RegisterDomainResponseStatusV20230925 {
    /// The domain is added, but not verified.
    Pending,
    /// The domain is added and verified. It can be used to accept Apple Pay transactions.
    Verified,
}
