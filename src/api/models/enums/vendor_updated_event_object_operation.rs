//! VendorUpdatedEventObjectOperation Enum

use serde::{Deserialize, Serialize};

/// The operation that can be performed against a vendor to cause the event to be published.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VendorUpdatedEventObjectOperation {
    Updated,
}
