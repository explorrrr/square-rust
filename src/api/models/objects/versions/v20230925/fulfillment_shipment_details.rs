//! FulfillmentShipmentDetails

use serde::{Deserialize, Serialize};

use super::fulfillment_recipient::FulfillmentRecipientV20230925;

/// Contains the details necessary to fulfill a shipment order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FulfillmentShipmentDetailsV20230925 {
    /// Information about the person to receive this shipment fulfillment.
    pub recipient: Option<FulfillmentRecipientV20230925>,
    /// The shipping carrier being used to ship this fulfillment (such as UPS, FedEx, or USPS).
    ///
    /// Max Length 50
    pub carrier: Option<String>,
    /// A note with additional information for the shipping carrier.
    ///
    /// Max Length 500
    pub shipping_note: Option<String>,
    /// A description of the type of shipping product purchased from the carrier (such as First Class, Priority, or Express).
    ///
    /// Max Length 50
    pub shipping_type: Option<String>,
    /// The reference number provided by the carrier to track the shipment's progress.
    ///
    /// Max Length 100
    pub tracking_number: Option<String>,
    /// A link to the tracking webpage on the carrier's website.
    ///
    /// Max Length 2000
    pub tracking_url: Option<String>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the shipment was requested. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub placed_at: Option<String>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when this fulfillment was moved to the RESERVED state, which indicates that preparation of this shipment has begun. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub in_progress_at: Option<String>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when this fulfillment was moved to the PREPARED state, which indicates that the fulfillment is packaged. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub packaged_at: Option<String>,
    /// The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the shipment is expected to be delivered to the shipping carrier. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub expected_shipped_at: Option<String>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when this fulfillment was moved to the COMPLETED state, which indicates that the fulfillment has been given to the shipping carrier. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub shipped_at: Option<String>,
    /// The timestamp indicating the shipment was canceled. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub canceled_at: Option<String>,
    /// A description of why the shipment was canceled.
    ///
    /// Max Length 100
    pub cancel_reason: Option<String>,
    /// Read only The [timestamp](https://developer.squareup.com/docs/build-basics/working-with-dates) indicating when the shipment failed to be completed. The timestamp must be in RFC 3339 format (for example, "2016-09-04T23:59:33.123Z").
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub failed_at: Option<String>,
    /// A description of why the shipment failed to be completed.
    ///
    /// Max Length 100
    pub failure_reason: Option<String>,
}
