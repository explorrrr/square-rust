//! PaymentLink

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentLink {
    /// The Square-assigned ID of the payment link.
    id: Option<String>,
    /// The Square-assigned version number, which is incremented each time an update is committed to the payment link.
    ///
    /// Max Length: 65535
    version: i32,
    /// The optional description of the payment_link object. It is primarily for use by your application and is not used anywhere.
    ///
    /// Max Length: 4096
    description: Option<String>,
    /// The ID of the order associated with the payment link.
    ///
    /// Max Length: 192
    order_id: Option<String>,
    /// The checkout options configured for the payment link. For more information, see Optional Checkout Configurations.
    checkout_options: Option<CheckoutOptions>,
    /// Describes buyer data to prepopulate on the checkout page.
    pre_populated_data: Option<PrePopulatedData>,
    /// The shortened URL of the payment link.
    ///
    /// Max Length: 255
    url: Option<String>,
    /// The long URL of the payment link.
    ///
    /// Max Length: 255
    long_url: Option<String>,
    /// The timestamp when the payment link was created, in RFC 3339 format.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    created_at: Option<String>,
    /// The timestamp when the payment link was last updated, in RFC 3339 format.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    updated_at: Option<String>,
    /// An optional note. After Square processes the payment, this note is added to the resulting Payment.
    ///
    /// Max Length: 500
    payment_note: Option<String>,
}
