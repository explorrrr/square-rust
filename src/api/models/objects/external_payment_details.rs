//! ExternalPaymentDetails

use serde::{Deserialize, Serialize};

/// Stores details about an external payment.
///
/// Contains only non-confidential information. For more information, see [Take External Payments](https://developer.squareup.com/docs/payments-api/take-payments/external-payments).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalPaymentDetails {
    /// REQUIRED The type of external payment the seller received. It can be one of the following:
    ///
    /// - CHECK - Paid using a physical check.
    /// - BANK_TRANSFER - Paid using external bank transfer.
    /// - OTHER_GIFT_CARD - Paid using a non-Square gift card.
    /// - CRYPTO - Paid using a crypto currency.
    /// - SQUARE_CASH - Paid using Square Cash App.
    /// - SOCIAL - Paid using peer-to-peer payment applications.
    /// - EXTERNAL - A third-party application gathered this payment outside of Square.
    /// - EMONEY - Paid using an E-money provider.
    /// - CARD - A credit or debit card that Square does not support.
    /// - STORED_BALANCE - Use for house accounts, store credit, and so forth.
    /// - FOOD_VOUCHER - Restaurant voucher provided by employers to employees to pay for meals
    /// - OTHER - A type not listed here.
    ///
    /// Max Length 50
    pub r#type: String,
    /// REQUIRED A description of the external payment source. For example, "Food Delivery Service".
    /// Max Length 255
    pub source: String,
    /// An ID to associate the payment to its originating source.
    /// Max Length 255
    pub source_id: Option<String>,
    /// The fees paid to the source. The amount_money minus this field is the net amount seller receives.
    pub source_fee_money: Option<Money>,
}
