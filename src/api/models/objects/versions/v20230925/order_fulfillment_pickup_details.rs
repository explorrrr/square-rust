//! OrderFulfillmentPickupDetails

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::order_fulfillment_pickup_details_schedule_type::OrderFulfillmentPickupDetailsScheduleTypeV20230925;

use super::{order_fulfillment_recipient::OrderFulfillmentRecipientV20230925, order_fulfillment_pickup_details_curbside_pickup_details::OrderFulfillmentPickupDetailsCurbsidePickupDetailsV20230925};

/// Contains details necessary to fulfill a pickup order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderFulfillmentPickupDetailsV20230925 {
    /// Information about the person to pick up this fulfillment from a physical location.
    pub recipient: Option<OrderFulfillmentRecipientV20230925>,
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when this fulfillment expires if it is not accepted. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z"). The expiration time can only be set up to 7 days in the future. If expires_at is not set, this pickup fulfillment is automatically accepted when placed.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub expires_at: Option<String>,
    /// The duration of time after which an open and accepted pickup fulfillment is automatically moved to the COMPLETED state. The duration must be in RFC 3339 format (for example, "P1W3D"). If not set, this pickup fulfillment remains accepted until it is canceled or completed.
    ///
    /// Example for 2 days, 12 hours, 30 minutes, and 15 seconds: P2DT12H30M15S
    pub auto_complete_duration: Option<String>,
    /// The schedule type of the pickup fulfillment. Defaults to SCHEDULED.
    pub schedule_type: Option<OrderFulfillmentPickupDetailsScheduleTypeV20230925>,
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) that represents the start of the pickup window. Must be in RFC 3339 timestamp format, e.g., "2016-09-04T23:59:33.123Z". For fulfillments with the schedule type ASAP, this is automatically set to the current time plus the expected duration to prepare the fulfillment.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub pickup_at: Option<String>,
    /// The window of time in which the order should be picked up after the pickup_at timestamp. Must be in RFC 3339 duration format, e.g., "P1W3D". Can be used as an informational guideline for merchants.
    ///
    /// Example for 2 days, 12 hours, 30 minutes, and 15 seconds: P2DT12H30M15S
    pub pickup_window_duration: Option<String>,
    /// The duration of time it takes to prepare this fulfillment. The duration must be in RFC 3339 format (for example, "P1W3D").
    ///
    /// Example for 2 days, 12 hours, 30 minutes, and 15 seconds: P2DT12H30M15S
    pub prep_time_duration: Option<String>,
    /// A note to provide additional instructions about the pickup fulfillment displayed in the Square Point of Sale application and set by the API.
    ///
    /// Max Length: 500
    pub note: Option<String>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was placed. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub placed_at: Option<String>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was accepted. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub accepted_at: Option<String>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was rejected. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub rejected_at: Option<String>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was marked as ready for pickup. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub ready_at: Option<String>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment expired. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub expired_at: Option<String>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was picked up by the recipient. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub picked_up_at: Option<String>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was canceled. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub canceled_at: Option<String>,
    /// A description of why the pickup was canceled. The maximum length: 100 characters.
    ///
    /// Max Length: 100
    pub cancel_reason: Option<String>,
    /// If set to true, indicates that this pickup order is for curbside pickup, not in-store pickup.
    pub is_curbside_pickup: Option<bool>,
    /// Specific details for curbside pickup. These details can only be populated if is_curbside_pickup is set to true.
    pub curbside_pickup_details: Option<OrderFulfillmentPickupDetailsCurbsidePickupDetailsV20230925>,
}
