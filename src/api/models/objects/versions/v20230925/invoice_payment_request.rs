//! InvoicePaymentRequest

use serde::{Deserialize, Serialize};

use crate::api::models::enums::versions::v20230925::{invoice_request_type::InvoiceRequestTypeV20230925, invoice_automatic_payment_source::InvoiceAutomaticPaymentSourceV20230925};

use super::{money::MoneyV20230925, invoice_payment_reminder::InvoicePaymentReminderV20230925};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicePaymentRequestV20230925 {
    /// The Square-generated ID of the payment request in an invoice.
    ///
    /// Min Length 1 Max Length 255
    pub uid: Option<String>,
    /// Identifies the payment request type. This type defines how the payment request amount is determined. This field is required to create a payment request.
    pub request_type: Option<InvoiceRequestTypeV20230925>,
    /// The due date (in the invoice's time zone) for the payment request, in YYYY-MM-DD format. This field is required to create a payment request. If an automatic_payment_source is defined for the request, Square charges the payment source on this date.
    ///
    /// After this date, the invoice becomes overdue. For example, a payment due_date of 2021-03-09 with a timezone of America/Los_Angeles becomes overdue at midnight on March 9 in America/Los_Angeles (which equals a UTC timestamp of 2021-03-10T08:00:00Z).
    pub due_date: Option<String>,
    /// If the payment request specifies DEPOSIT or INSTALLMENT as the request_type, this indicates the request amount. You cannot specify this when request_type is BALANCE or when the payment request includes the percentage_requested field.
    pub fixed_amount_requested_money: Option<MoneyV20230925>,
    /// Specifies the amount for the payment request in percentage:
    ///
    /// - When the payment request_type is DEPOSIT, it is the percentage of the order's total amount.
    /// - When the payment request_type is INSTALLMENT, it is the percentage of the order's total less the deposit, if requested. The sum of the
    ///
    /// You cannot specify this when the payment request_type is BALANCE or when the payment request specifies the fixed_amount_requested_money field.
    pub percentage_requested: Option<String>,
    /// If set to true, the Square-hosted invoice page (the public_url field of the invoice) provides a place for the customer to pay a tip.
    ///
    /// This field is allowed only on the final payment request and the payment request_type must be BALANCE or INSTALLMENT.
    pub tipping_enabled: Option<bool>,
    /// The payment method for an automatic payment.
    ///
    /// The default value is NONE.
    pub automatic_payment_source: Option<InvoiceAutomaticPaymentSourceV20230925>,
    /// The ID of the credit or debit card on file to charge for the payment request. To get the cards on file for a customer, call ListCards and include the customer_id of the invoice recipient.
    ///
    /// Min Length 1 Max Length 255
    pub card_id: Option<String>,
    /// A list of one or more reminders to send for the payment request.
    pub reminders: Option<Vec<InvoicePaymentReminderV20230925>>,
    /// Read only The amount of the payment request, computed using the order amount and information from the various payment request fields (request_type, fixed_amount_requested_money, and percentage_requested).
    pub computed_amount_money: Option<MoneyV20230925>,
    /// Read only The amount of money already paid for the specific payment request. This amount might include a rounding adjustment if the most recent invoice payment was in cash in a currency that rounds cash payments (such as, CAD or AUD).
    pub total_completed_amount_money: Option<MoneyV20230925>,
    /// Read only If the most recent payment was a cash payment in a currency that rounds cash payments (such as, CAD or AUD) and the payment is rounded from computed_amount_money in the payment request, then this field specifies the rounding adjustment applied. This amount might be negative.
    pub rounding_adjustment_included_money: Option<MoneyV20230925>,
}
