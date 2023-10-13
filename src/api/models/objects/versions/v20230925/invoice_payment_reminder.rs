//! InvoicePaymentReminder

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::invoice_payment_reminder_status::InvoicePaymentReminderStatusV20230925;

/// Describes a payment request reminder (automatic notification) that Square sends to the customer.
///
/// You configure a reminder relative to the payment request due_date.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicePaymentReminderV20230925 {
    /// A Square-assigned ID that uniquely identifies the reminder within the InvoicePaymentRequest.
    pub uid: Option<String>,
    /// The number of days before (a negative number) or after (a positive number) the payment request due_date when the reminder is sent. For example, -3 indicates that the reminder should be sent 3 days before the payment request due_date.
    ///
    /// Min: -32767 Max: 32767
    pub relative_scheduled_days: Option<i32>,
    /// The reminder message.
    ///
    /// Min Length: 1 Max Length: 1000
    pub message: Option<String>,
    /// The status of the reminder.
    pub status: Option<InvoicePaymentReminderStatusV20230925>,
    /// If sent, the timestamp when the reminder was sent, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub sent_at: Option<String>,
}
