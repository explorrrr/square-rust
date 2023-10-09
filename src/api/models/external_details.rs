//! ExternalDetails

use serde::{Deserialize, Serialize};

use crate::api::models::money::Money;

/// Stores details about an external payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ExternalDetails {
    /// The type of external payment the seller received.
    pub type_: ExternalDetailsType,
    /// A description of the external payment source. For example, "Food Delivery Service".
    pub source: String,
    /// An ID to associate this payment to its originating source.
    pub id: Option<String>,
    /// The fees paid to the source. The amount_money minus this field is the net amount sellers receive.
    pub source_fee_money: Option<Money>,
}

/// Indicates the type of external payment the seller received.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExternalDetailsType {
    /// Paid using a physical check.
    Check,
    /// Paid using external bank transfer.
    BankTransfer,
    /// Paid using a non-Square gift card.
    OtherGiftCard,
    /// Paid using a crypto currency.
    Crypto,
    /// Paid using Square Cash App.
    SquareCash,
    /// Paid using peer-to-peer payment applications.
    Social,
    /// A third-party application gathered this payment outside of Square.
    External,
    /// Paid using an E-money provider.
    EMoney,
    /// A credit or debit card that Square does not support.
    Card,
    /// Use for house accounts, store credit, and so forth.
    StoredBalance,
    /// Restaurant voucher provided by employers to employees to pay for meals
    FoodVoucher,
    /// A type not listed here.
    Other,
}
