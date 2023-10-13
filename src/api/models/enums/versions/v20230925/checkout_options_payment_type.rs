//! CheckoutOptionsPaymentType Enum

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CheckoutOptionsPaymentTypeV20230925 {
    /// Accept credit card or debit card payments via tap, dip or swipe.
    CardPresent,
    /// Launches the manual credit or debit card entry screen for the buyer to complete.
    ManualCardEntry,
    /// Launches the iD checkout screen for the buyer to complete.
    FelicaId,
    /// Launches the QUICPay checkout screen for the buyer to complete.
    FelicaQuicpay,
    /// Launches the Transportation Group checkout screen for the buyer to complete.
    FelicaTransportationGroup,
    /// Launches a checkout screen for the buyer on the Square Terminal that allows them to select a specific FeliCa brand or select the check balance screen.
    FelicaAll,
    /// Launches the PayPay checkout screen for the buyer to complete.
    Paypay,
}
