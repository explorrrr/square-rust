//! TenderType Enum

use serde::{Deserialize, Serialize};

/// Indicates a tender's type.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TenderTypeV20230925 {
    /// A credit card.
    Card,
    /// Cash.
    Cash,
    /// A credit card processed with a card processor other than Square.
    ///
    /// This value applies only to merchants in countries where Square does not yet provide card processing.
    ThirdPartyCard,
    /// A Square gift card.
    SquareGiftCard,
    /// This tender represents the register being opened for a "no sale" event.
    NoSale,
    /// A bank account payment.
    BankAccount,
    /// A payment from a digital wallet, e.g. Cash App.
    ///
    /// Note: Some "digital wallets", including Google Pay and Apple Pay, facilitate card payments. Those payments have the CARD type.
    Wallet,
    /// A Buy Now Pay Later payment.
    BuyNowPayLater,
    /// A Square House Account payment.
    SquareAccount,
    /// A form of tender that does not match any other value.
    Other,
}
