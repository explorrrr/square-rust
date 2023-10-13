//! OauthAuthorizationRevokedEventRevokerType Enum

use serde::{Deserialize, Serialize};

/// Defines the possible types for the revoking client.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OauthAuthorizationRevokedEventRevokerType {
    /// The application that requested access to a merchant's data.
    Application,
    /// The admin for the merchant.
    Merchant,
    /// An internal Square employee.
    Square,
}
