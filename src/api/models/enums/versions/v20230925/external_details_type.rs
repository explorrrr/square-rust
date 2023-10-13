use serde::{Deserialize, Serialize};

/// Indicates the type of external payment the seller received.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExternalDetailsTypeV20230925 {
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
