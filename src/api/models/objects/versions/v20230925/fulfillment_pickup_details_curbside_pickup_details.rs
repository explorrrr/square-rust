//! FulfillmentPickupDetailsCurbsidePickupDetails

use serde::{Deserialize, Serialize};

/// Specific details for curbside pickup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FulfillmentPickupDetailsCurbsidePickupDetailsV20230925 {
    /// Specific details for curbside pickup, such as parking number and vehicle model.
    ///
    /// Max Length 250
    pub curbside_details: Option<String>,
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the buyer arrived and is waiting for pickup. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub buyer_arrived_at: Option<String>,
}
