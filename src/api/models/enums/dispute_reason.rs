//! DisputeReason enum

use serde::{Deserialize, Serialize};

/// The list of possible reasons why a cardholder might initiate a dispute with their bank.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DisputeReason {
    /// The cardholder claims that they were charged the wrong amount for the purchase. To challenge this dispute, provide specific and concrete evidence that the cardholder agreed to the amount charged.
    AmountDiffers,
    /// The cardholder claims that they attempted to return the goods or cancel the service. To challenge this dispute, provide specific and concrete evidence to prove that the cardholder is not due a refund and that the cardholder acknowledged your cancellation policy.
    Cancelled,
    /// The cardholder claims that they were charged twice for the same purchase. To challenge this dispute, provide specific and concrete evidence that shows both charges are legitimate and independent of one another.
    Duplicate,
    /// The cardholder claims that they did not make this purchase nor authorized the charge. To challenge this dispute, provide specific and concrete evidence that proves that the cardholder identity was verified at the time of purchase and that the purchase was authorized.
    NoKnowledge,
    /// The cardholder claims the product or service was provided, but the quality of the deliverable did not align with the expectations of the cardholder based on the description. To challenge this dispute, provide specific and concrete evidence that shows the cardholder is in possession of the product as described or received the service as described and agreed on.
    NotAsDescribed,
    /// The cardholder claims the product or service was not received by the cardholder within the stated time frame. To challenge this dispute, provide specific and concrete evidence to prove that the cardholder is in possession of or received the product or service sold.
    NotReceived,
    /// The cardholder claims that they previously paid for this purchase. To challenge this dispute, provide specific and concrete evidence that shows both charges are legitimate and independent of one another or proof that you already provided a credit for the charge.
    PaidByOtherMeans,
    /// The cardholder claims that the purchase was canceled or returned, but they have not yet received the credit. To challenge this dispute, provide specific and concrete evidence to prove that the cardholder is not due a refund and that they acknowledged your cancellation and/or refund policy.
    CustomerRequestsCredit,
    /// A chip-enabled card was not processed through a compliant chip-card reader (for example, it was swiped instead of dipped into a chip-card reader). You cannot challenge this dispute because the payment did not comply with EMV security requirements. For more information, see [What Is EMV?](https://squareup.com/emv)
    EmvLiabilityShift,
}
