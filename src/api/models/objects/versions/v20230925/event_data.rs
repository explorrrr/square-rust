//! EventData

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDataV20230925 {
    /// Name of the affected object’s type.
    pub type_: Option<String>,
    /// ID of the affected object.
    pub id: Option<String>,
    /// Is true if the affected object was deleted. Otherwise absent.
    pub deleted: Option<bool>,
    /// An object containing fields and values relevant to the event. Is absent if affected object was deleted.
    pub object: Option<serde_json::Value>,
}
