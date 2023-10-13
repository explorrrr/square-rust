//! EventTypeMetadata

use serde::{Deserialize, Serialize};

/// Contains the metadata of a webhook event type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTypeMetadata {
    /// Read only The event type.
    pub event_type: Option<String>,
    /// Read only The API version at which the event type was introduced.
    pub api_version_introduced: Option<String>,
    /// Read only The release status of the event type.
    pub release_status: Option<String>,
}
