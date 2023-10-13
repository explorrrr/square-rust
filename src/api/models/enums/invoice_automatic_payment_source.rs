//! InvoiceAutomaticPaymentSource Enum

use serde::{Deserialize, Serialize};

/// Indicates the automatic payment method for an [invoice payment request](https://developer.squareup.com/reference/square/objects/InvoicePaymentRequest).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceAutomaticPaymentSource {
    /// An automatic payment is not configured for the payment request.
    None,
    /// Use a card on file as the automatic payment method. On the due date, Square charges the card for the amount of the payment request.
    ///
    /// For `CARD_ON_FILE` payments, the invoice delivery method must be `EMAIL` and `card_id` must be specified for the payment request before the invoice can be published.
    CardOnFile,
    /// Use a bank account on file as the automatic payment method. On the due date, Square charges the bank account for the amount of the payment request if the buyer has approved the payment. The buyer receives a request to approve the payment when the invoice is sent or the invoice is updated.
    ///
    /// This payment method applies only to invoices that sellers create in the Seller Dashboard or other Square product. The bank account is provided by the customer during the payment flow.
    ///
    /// You cannot set `BANK_ON_FILE` as a payment method using the Invoices API, but you can change a `BANK_ON_FILE` payment method to `NONE` or `CARD_ON_FILE`. For `BANK_ON_FILE` payments, the invoice delivery method must be `EMAIL`.
    BankOnFile,
}
