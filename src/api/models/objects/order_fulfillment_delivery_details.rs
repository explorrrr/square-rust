//! OrderFulfillmentDeliveryDetails

use serde::{Deserialize, Serialize};

/// Describes delivery details of an order fulfillment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderFulfillmentDeliveryDetails {
    /// The contact information for the person to receive the fulfillment.
    pub recipient: Option<OrderFulfillmentRecipient>,
    /// Indicates the fulfillment delivery schedule type. If SCHEDULED, then deliver_at is required. If ASAP, then prep_time_duration is required. The default is SCHEDULED.
    pub schedule_type: Option<OrderFulfillmentDeliveryDetailsScheduleType>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was placed. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z"). Must be in RFC 3339 timestamp format, e.g., "2016-09-04T23:59:33.123Z".
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub placed_at: Option<String>,
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) that represents the start of the delivery period. When the fulfillment schedule_type is ASAP, the field is automatically set to the current time plus the prep_time_duration. Otherwise, the application can set this field while the fulfillment state is PROPOSED, RESERVED, or PREPARED (any time before the terminal state such as COMPLETED, CANCELED, and FAILED).
    ///
    /// The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub deliver_at: Option<String>,
    /// The duration of time it takes to prepare and deliver this fulfillment. The timestamp must be in RFC 3339 format (for example, "P1W3D").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub prep_time_duration: Option<String>,
    /// The time period after the deliver_at timestamp in which to deliver the order. Applications can set this field when the fulfillment state is PROPOSED, RESERVED, or PREPARED (any time before the terminal state such as COMPLETED, CANCELED, and FAILED). The timestamp must be in RFC 3339 format (for example, "P1W3D").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub delivery_window_duration: Option<String>,
    /// Provides additional instructions about the delivery fulfillment. It is displayed in the Square Point of Sale application and set by the API.
    ///
    /// Max Length: 550
    pub note: Option<String>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the seller completed the fulfillment. This field is automatically set when fulfillment state changes to COMPLETED. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub completed_at: Option<String>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicates when the seller started processing the fulfillment. This field is automatically set when the fulfillment state changes to RESERVED. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub in_progress_at: Option<String>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was rejected. This field is automatically set when the fulfillment state changes to FAILED. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub rejected_at: Option<String>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the seller marked the fulfillment as ready for courier pickup. This field is automatically set when the fulfillment state changes to PREPARED. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub ready_at: Option<String>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was delivered to the recipient. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub delivered_at: Option<String>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the fulfillment was canceled. This field is automatically set when the fulfillment state changes to CANCELED.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub canceled_at: Option<String>,
    /// The delivery cancellation reason.
    ///
    /// Max Length: 100
    pub cancel_reason: Option<String>,
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when an order can be picked up by the courier for delivery. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub courier_pickup_at: Option<String>,
    /// The period of time in which the order should be picked up by the courier after the courier_pickup_at timestamp. The time must be in RFC 3339 format (for example, "P1W3D").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub courier_pickup_window_duration: Option<String>,
    /// Whether the delivery is preferred to be no contact.
    pub is_no_contact_delivery: Option<bool>,
    /// A note to provide additional instructions about how to deliver the order.
    ///
    /// Max Length: 550
    pub dropoff_notes: Option<String>,
    /// The name of the courier provider.
    ///
    /// Max Length: 255
    pub courier_provider_name: Option<String>,
    /// The support phone number of the courier.
    ///
    /// Max Length: 17
    pub courier_support_phone_number: Option<String>,
    /// The identifier for the delivery created by Square.
    ///
    /// Max Length: 50
    pub square_delivery_id: Option<String>,
    /// The identifier for the delivery created by the third-party courier service.
    ///
    /// Max Length: 50
    pub external_delivery_id: Option<String>,
    /// The flag to indicate the delivery is managed by a third party (ie DoorDash), which means we may not receive all recipient information for PII purposes.
    pub managed_delivery: Option<bool>,
}
