//! Event

use serde::{Deserialize, Serialize};

use super::event_data::EventDataV20230925;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventV20230925 {
    /// The ID of the target merchant associated with the event.
    pub merchant_id: Option<String>,
    /// The ID of the location associated with the event.
    pub location_id: Option<String>,
    /// The type of event this represents.
    pub r#type: Option<String>,
    /// A unique ID for the event.
    pub event_id: Option<String>,
    /// Read only Timestamp of when the event was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: Option<String>,
    /// Data associated with the event.
    pub data: Option<EventDataV20230925>,
}
