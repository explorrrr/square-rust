//! Models for CustomerPreferences
use serde::{Deserialize, Serialize};

/// Represents general customer preferences.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CustomerPreferencesV20230925 {
    /// Indicates whether the customer has unsubscribed from marketing campaign emails. A value of true means that the customer chose to opt out of email marketing from the current Square seller or from all Square sellers. This value is read-only from the Customers API.
    pub email_unsubscribed: Option<bool>,
}
