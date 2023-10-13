//! InvoiceDeliveryMethod Enum

use serde::{Deserialize, Serialize};

/// Indicates how Square delivers the [invoice](https://developer.squareup.com/reference/square/objects/Invoice) to the customer.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceDeliveryMethodV20230925 {
    /// Directs Square to send invoices, reminders, and receipts to the customer using email.
    Email,
    /// Directs Square to take no action on the invoice. In this case, the seller or application developer follows up with the customer for payment. For example, a seller might collect a payment in the Seller Dashboard or Point of Sale (POS) application. The seller might also share the URL of the Square-hosted invoice page (public_url) with the customer to request payment.
    ShareManually,
    /// Directs Square to send invoices and receipts to the customer using SMS (text message).
    Sms,
}
