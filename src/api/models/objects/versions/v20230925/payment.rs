//! Payment

use serde::{Deserialize, Serialize};

use super::{
    address::AddressV20230925, application_details::ApplicationDetailsV20230925,
    bank_account_payment_details::BankAccountPaymentDetailsV20230925,
    buy_now_pay_later_details::BuyNowPayLaterDetailsV20230925, card_payment_details::CardPaymentDetailsV20230925,
    cash_payment_details::CashPaymentDetailsV20230925, device_details::DeviceDetailsV20230925,
    digital_wallet_details::DigitalWalletDetailsV20230925, external_payment_details::ExternalPaymentDetailsV20230925,
    money::MoneyV20230925, processing_fee::ProcessingFeeV20230925, risk_evaluation::RiskEvaluationV20230925,
    square_account_details::SquareAccountDetailsV20230925,
};

/// Represents a payment processed by the Square API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentV20230925 {
    /// Read only A unique ID for the payment.
    ///
    /// Max Length 192
    pub id: Option<String>,
    /// Read only The timestamp of when the payment was created, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    ///
    /// Max Length 32
    pub created_at: Option<String>,
    /// Read only The timestamp of when the payment was last updated, in RFC 3339 format.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    ///
    /// Max Length 32
    pub updated_at: Option<String>,
    /// The amount processed for this payment, not including tip_money.
    ///
    /// This amount is specified in the smallest denomination of the applicable currency (for example, US dollar amounts are specified in cents). For more information, see [Working with Monetary Amounts](https://developer.squareup.com/docs/build-basics/working-with-monetary-amounts).
    pub amount_money: Option<MoneyV20230925>,
    /// The amount designated as a tip.
    ///
    /// This amount is specified in the smallest denomination of the applicable currency (for example, US dollar amounts are specified in cents). For more information, see [Working with Monetary Amounts](https://developer.squareup.com/docs/build-basics/working-with-monetary-amounts).
    pub tip_money: Option<MoneyV20230925>,
    /// Read only The total amount for the payment, including amount_money and tip_money. This amount is specified in the smallest denomination of the applicable currency (for example, US dollar amounts are specified in cents). For more information, see [Working with Monetary Amounts](https://developer.squareup.com/docs/build-basics/working-with-monetary-amounts).
    pub total_money: Option<MoneyV20230925>,
    /// The amount the developer is taking as a fee for facilitating the payment on behalf of the seller. This amount is specified in the smallest denomination of the applicable currency (for example, US dollar amounts are specified in cents). For more information, see [Take Payments and Collect Fees](https://developer.squareup.com/docs/payments-api/take-payments-and-collect-fees).
    ///
    /// The amount cannot be more than 90% of the total_money value.
    ///
    /// To set this field, `PAYMENTS_WRITE_ADDITIONAL_RECIPIENTS` OAuth permission is required. For more information, see [Permissions](https://developer.squareup.com/docs/payments-api/take-payments-and-collect-fees#permissions).
    pub app_fee_money: Option<MoneyV20230925>,
    /// The initial amount of money approved for this payment.
    pub approved_money: Option<MoneyV20230925>,
    /// Read only The processing fees and fee adjustments assessed by Square for this payment.
    pub processing_fee: Option<Vec<ProcessingFeeV20230925>>,
    /// Read only The total amount of the payment refunded to date.
    ///
    /// This amount is specified in the smallest denomination of the applicable currency (for example, US dollar amounts are specified in cents).
    pub refunded_money: Option<MoneyV20230925>,
    /// Read only Indicates whether the payment is `APPROVED`, `PENDING`, `COMPLETED`, `CANCELED`, or `FAILED`.
    ///
    /// Max Length 50
    pub status: Option<String>,
    /// Read only The duration of time after the payment's creation when Square automatically applies the `delay_action` to the payment. This automatic `delay_action` applies only to payments that do not reach a terminal state (`COMPLETED`, `CANCELED`, or `FAILED`) before the `delay_duration` time period.
    ///
    /// This field is specified as a time duration, in RFC 3339 format.
    ///
    /// **Notes:** This feature is only supported for card payments.
    /// Default:
    /// - Card-present payments: `"PT36H"` (36 hours) from the creation time.
    /// - Card-not-present payments: `"P7D"` (7 days) from the creation time.
    ///
    /// Example for 2 days, 12 hours, 30 minutes, and 15 seconds: `P2DT12H30M15S`
    pub delay_duration: Option<String>,
    /// The action to be applied to the payment when the `delay_duration` has elapsed.
    ///
    /// Current values include `CANCEL` and `COMPLETE`.
    pub delay_action: Option<String>,
    /// Read only The read-only timestamp of when the `delay_action` is automatically applied, in RFC 3339 format.
    ///
    /// Note that this field is read-only. To set this field, use the `delay_action` field in the `CreatePayment` request.
    ///
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    ///
    /// UTC: 2020-01-26T02:25:34Z
    ///
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub delayed_until: Option<String>,
    /// Read only The read-only timestamp of when the `delay_action` is automatically applied, in RFC 3339 format.
    ///
    /// Current values include `CARD`, `BANK_ACCOUNT`, `WALLET`, `BUY_NOW_PAY_LATER`, `SQUARE_ACCOUNT`, `CASH` and `EXTERNAL`. For information about these payment source types, see [Take Payments](https://developer.squareup.com/docs/payments-api/take-payments).
    ///
    /// Max Length 50
    pub source_type: Option<String>,
    /// Read only Details about a card payment. These details are only populated if the `source_type` is `CARD`.
    pub card_details: Option<CardPaymentDetailsV20230925>,
    /// Details about a cash payment. These details are only populated if the `source_type` is `CASH`.
    pub cash_details: Option<CashPaymentDetailsV20230925>,
    /// Read only Details about a bank account payment. These details are only populated if the `source_type` is `BANK_ACCOUNT`.
    pub bank_account_details: Option<BankAccountPaymentDetailsV20230925>,
    /// Read only Details about an external payment. The details are only populated if the `source_type` is `EXTERNAL`.
    pub external_details: Option<ExternalPaymentDetailsV20230925>,
    /// Read only Details about an wallet payment. The details are only populated if the `source_type` is `WALLET`.
    pub wallet_details: Option<DigitalWalletDetailsV20230925>,
    /// Read only Details about a Buy Now Pay Later payment. The details are only populated if the `source_type` is `BUY_NOW_PAY_LATER`. For more information, see [Afterpay Payments](https://developer.squareup.com/docs/payments-api/take-payments/afterpay-payments).
    pub buy_now_pay_later_details: Option<BuyNowPayLaterDetailsV20230925>,
    /// Read only Details about a Square Account payment. The details are only populated if the `source_type` is `SQUARE_ACCOUNT`.
    pub square_account_details: Option<SquareAccountDetailsV20230925>,
    /// Read only The ID of the location associated with the payment.
    ///
    /// Max Length 50
    pub location_id: Option<String>,
    /// Read only The ID of the order associated with the payment.
    ///
    /// Max Length 192
    pub order_id: Option<String>,
    /// Read only An optional ID that associates the payment with an entity in another system.
    ///
    /// Max Length 40
    pub reference_id: Option<String>,
    /// Read only The ID of the customer associated with the payment. If the ID is not provided in the `CreatePayment` request that was used to create the Payment, Square may use information in the request (such as the billing and shipping address, email address, and payment source) to identify a matching customer profile in the Customer Directory. If found, the profile ID is used. If a profile is not found, the API attempts to create an instant profile. If the API cannot create an [instant profile](https://developer.squareup.com/docs/customers-api/what-it-does#instant-profiles) (either because the seller has disabled it or the seller's region prevents creating it), this field remains unset. Note that this process is asynchronous and it may take some time before a customer ID is added to the payment.
    ///
    /// Max Length 191
    pub customer_id: Option<String>,
    /// Read only An optional ID of the [TeamMember](https://developer.squareup.com/reference/square/objects/TeamMember) associated with taking the payment.
    ///
    /// Max Length 192
    pub team_member_id: Option<String>,
    /// Read only A list of `refund_ids` identifying refunds for the payment.
    pub refund_ids: Option<Vec<String>>,
    /// Read only Provides information about the risk associated with the payment, as determined by Square. This field is present for payments to sellers that have opted in to receive risk evaluations.
    pub risk_evaluation: Option<RiskEvaluationV20230925>,
    /// Read only The buyer's email address.
    ///
    /// Max Length 255
    pub buyer_email_address: Option<String>,
    /// Read only The buyer's billing address.
    pub billing_address: Option<AddressV20230925>,
    /// Read only The buyer's shipping address.
    pub shipping_address: Option<AddressV20230925>,
    /// Read only An optional note to include when creating a payment.
    ///
    /// Max Length 500
    pub note: Option<String>,
    /// Read only Additional payment information that gets added to the customer's card statement as part of the statement description.
    ///
    /// Note that the `statement_description_identifier` might get truncated on the statement description to fit the required information including the Square identifier (SQ *) and name of the seller taking the payment.
    pub statement_description_identifier: Option<String>,
    /// Read only Actions that can be performed on this payment:
    ///
    /// - `EDIT_AMOUNT_UP` - The payment amount can be edited up.
    /// - `EDIT_AMOUNT_DOWN` - The payment amount can be edited down.
    /// - `EDIT_TIP_AMOUNT_UP` - The tip amount can be edited up.
    /// - `EDIT_TIP_AMOUNT_DOWN` - The tip amount can be edited down.
    /// - `EDIT_DELAY_ACTION` - The delay_action can be edited.
    pub actions: Option<Vec<String>>,
    /// Read only The payment's receipt number. The field is missing if a payment is canceled.
    ///
    /// Max Length 4
    pub receipt_number: Option<String>,
    /// Read only The URL for the payment's receipt. The field is only populated for COMPLETED payments.
    ///
    /// Max Length 255
    pub receipt_url: Option<String>,
    /// Read only Details about the device that took the payment.
    pub device_details: Option<DeviceDetailsV20230925>,
    /// Read only Details about the application that took the payment.
    pub application_details: Option<ApplicationDetailsV20230925>,
    /// Used for optimistic concurrency. This opaque token identifies a specific version of the Payment object.
    pub version_token: Option<String>,
}
