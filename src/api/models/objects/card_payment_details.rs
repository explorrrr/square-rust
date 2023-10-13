//! CardPaymentDetails

use serde::{Deserialize, Serialize};

/// Reflects the current status of a card payment.
///
/// Contains only non-confidential information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardPaymentDetails {
    /// The card payment's current state. The state can be AUTHORIZED, CAPTURED, VOIDED, or FAILED.
    /// Max Length 50
    pub status: Option<String>,
    /// The credit card's non-confidential details.
    pub card: Option<Card>,
    /// The method used to enter the card's details for the payment. The method can be KEYED, SWIPED, EMV, ON_FILE, or CONTACTLESS.
    /// Max Length 50
    pub entry_method: Option<String>,
    /// The status code returned from the Card Verification Value (CVV) check. The code can be CVV_ACCEPTED, CVV_REJECTED, or CVV_NOT_CHECKED.
    /// Max Length 50
    pub cvv_status: Option<String>,
    /// The status code returned from the Address Verification System (AVS) check. The code can be AVS_ACCEPTED, AVS_REJECTED, or AVS_NOT_CHECKED.
    /// Max Length 50
    pub avs_status: Option<String>,
    /// The status code returned by the card issuer that describes the payment's authorization status.
    /// Max Length 10
    pub auth_result_code: Option<String>,
    /// For EMV payments, the application ID identifies the EMV application used for the payment.
    /// Max Length 32
    pub application_identifier: Option<String>,
    /// For EMV payments, the human-readable name of the EMV application used for the payment.
    /// Max Length 16
    pub application_name: Option<String>,
    /// For EMV payments, the cryptogram generated for the payment.
    /// Max Length 16
    pub application_cryptogram: Option<String>,
    /// For EMV payments, the method used to verify the cardholder's identity. The method can be PIN, SIGNATURE, PIN_AND_SIGNATURE, ON_DEVICE, or NONE.
    /// Max Length 50
    pub verification_method: Option<String>,
    /// For EMV payments, the results of the cardholder verification. The result can be SUCCESS, FAILURE, or UNKNOWN.
    /// Max Length 50
    pub verification_results: Option<String>,
    /// The statement description sent to the card networks.
    ///
    /// Note: The actual statement description varies and is likely to be truncated and appended with additional information on a per issuer basis.
    /// Max Length 50
    pub statement_description: Option<String>,
    /// The timeline for card payments.
    pub card_payment_timeline: Option<CardPaymentTimeline>,
    /// Whether the card must be physically present for the payment to be refunded. If set to true, the card must be present.
    pub refund_requires_card_presence: Option<bool>,
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<SquareError>>,
}
